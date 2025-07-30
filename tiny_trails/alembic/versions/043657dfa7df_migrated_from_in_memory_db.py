"""migrated from in memory db

Revision ID: 043657dfa7df
Revises:
Create Date: 2025-07-27 15:47:44.652006

"""

from typing import Sequence, Union

import sqlalchemy as sa
from alembic import op

from tiny_trails.endpoints.common.models import TRAIL_TOKEN_LENGTH

# revision identifiers, used by Alembic.
revision: str = "043657dfa7df"
down_revision: Union[str, Sequence[str], None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    # it is just generally better to use pgcrypto for random string than using built-in random
    op.execute("CREATE EXTENSION IF NOT EXISTS pgcrypto;")

    # try to use as many function as possible on postgres side, to speed things up
    # (and also it makes sense to do so, as it is completely related to the database)
    op.execute(
        """
CREATE OR REPLACE FUNCTION base52_encode(num BIGINT) RETURNS TEXT AS $$
DECLARE
    alphabet TEXT := 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
    base INT := 52;
    result TEXT := '';
    remainder INT;
BEGIN
    IF num < 0 THEN
        RAISE EXCEPTION 'Only non-negative integers allowed';
    END IF;

    IF num = 0 THEN
        RETURN substr(alphabet, 1, 1);  -- 'a'
    END IF;

    WHILE num > 0 LOOP
        remainder := (num % base)::INT;
        result := substr(alphabet, remainder + 1, 1) || result;
        num := num / base;
    END LOOP;

    RETURN result;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT;
        """
    )

    # a simple random string generator, using pgcrypto for randomness
    op.execute(
        """
CREATE OR REPLACE FUNCTION random_string(length INT) RETURNS TEXT AS $$
DECLARE
    chars TEXT := 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    result TEXT := '';
    i INT;
    idx INT;
BEGIN
    FOR i IN 1..length LOOP
        idx := get_byte(gen_random_bytes(1), 0) % length(chars) + 1;
        result := result || substr(chars, idx, 1);
    END LOOP;

    RETURN result;
END;
$$ LANGUAGE plpgsql VOLATILE;
        """
    )

    # this function generates a random token of TRAIL_TOKEN_LENGTH characters
    op.execute(
        f"""
CREATE OR REPLACE FUNCTION generate_token() RETURNS TEXT AS $$
BEGIN
    RETURN random_string({TRAIL_TOKEN_LENGTH});
END;
$$ LANGUAGE plpgsql VOLATILE;
        """
    )

    op.create_table(
        "trails",
        sa.Column("id", sa.Integer(), autoincrement=True, nullable=False),
        sa.Column("trail_id", sa.String(), nullable=False),
        sa.Column("url", sa.String(), nullable=False),
        sa.Column(
            "token",
            sa.VARCHAR(TRAIL_TOKEN_LENGTH),
            nullable=False,
            server_default=sa.text("generate_token()"),
        ),
        sa.Column("lifetime", sa.Integer(), nullable=False),
        sa.Column(
            "created_at", sa.DateTime(), server_default=sa.text("now()"), nullable=False
        ),
        sa.PrimaryKeyConstraint("id"),
    )

    # probably we will query by token and trail_id often, so let's index them
    op.create_index(op.f("ix_trails_token"), "trails", ["token"], unique=False)
    op.create_index(op.f("ix_trails_trail_id"), "trails", ["trail_id"], unique=True)

    # it is efficient and convenient to set trail_id before inserting a new trail,
    # so we will use a trigger to set it automatically
    op.execute(
        """
CREATE OR REPLACE FUNCTION set_trail_id() RETURNS TRIGGER AS $$
BEGIN
    NEW.trail_id := base52_encode(NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_set_trail_id
BEFORE INSERT ON trails
FOR EACH ROW
WHEN (NEW.trail_id IS NULL)
EXECUTE FUNCTION set_trail_id();
        """
    )

    op.create_table(
        "peeks",
        sa.Column("id", sa.Integer(), autoincrement=True, nullable=False),
        sa.Column("trail_id", sa.Integer(), nullable=False),
        sa.Column(
            "created_at", sa.DateTime(), server_default=sa.text("now()"), nullable=False
        ),
        sa.ForeignKeyConstraint(["trail_id"], ["trails.id"], ondelete="CASCADE"),
        sa.PrimaryKeyConstraint("id"),
    )

    op.create_table(
        "visits",
        sa.Column("id", sa.Integer(), autoincrement=True, nullable=False),
        sa.Column("trail_id", sa.Integer(), nullable=False),
        sa.Column("hashed_ip", sa.String(), nullable=False),
        sa.Column(
            "created_at", sa.DateTime(), server_default=sa.text("now()"), nullable=False
        ),
        sa.ForeignKeyConstraint(["trail_id"], ["trails.id"], ondelete="CASCADE"),
        sa.PrimaryKeyConstraint("id"),
    )


def downgrade() -> None:
    op.drop_table("visits")
    op.drop_table("peeks")
    op.drop_index(op.f("ix_trails_trail_id"), table_name="trails")
    op.drop_index(op.f("ix_trails_token"), table_name="trails")
    op.drop_table("trails")

    op.execute(
        """
DROP TRIGGER IF EXISTS trg_set_trail_id ON trails;
DROP FUNCTION IF EXISTS set_trail_id() CASCADE;
DROP FUNCTION IF EXISTS base52_encode(num BIGINT) CASCADE;
DROP FUNCTION IF EXISTS random_string(length INT) CASCADE;
DROP FUNCTION IF EXISTS generate_token() CASCADE;
DROP EXTENSION IF EXISTS pgcrypto CASCADE;
        """
    )
