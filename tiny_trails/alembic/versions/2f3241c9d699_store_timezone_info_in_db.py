"""store timezone info in db

Revision ID: 2f3241c9d699
Revises: 043657dfa7df
Create Date: 2025-08-02 02:30:33.348261

"""

from datetime import datetime
from logging import getLogger
from typing import Sequence, Union

import sqlalchemy as sa
from alembic import op

# revision identifiers, used by Alembic.
revision: str = "2f3241c9d699"
down_revision: Union[str, Sequence[str], None] = "043657dfa7df"
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None

logger = getLogger(revision)


def _get_utc_offset() -> str:
    offset = datetime.now().astimezone().utcoffset()
    if offset is None:
        logger.warning(
            "Could not determine UTC offset, setting it to +00. "
            "This may offset 'created_at' column values by the local timezone offset."
        )
        sign = "+"
        hours = 0
        minutes = 0
    else:
        total_minutes = int(offset.total_seconds() // 60)

        # opposite sign, as we want to unshift the timestamp to UTC+00
        sign = "-" if total_minutes >= 0 else "+"

        hours = abs(total_minutes) // 60
        minutes = abs(total_minutes) % 60

    return f"'{sign}{hours:02}:{minutes:02}'"


def upgrade() -> None:
    conn = op.get_bind()
    utc_offset_str = _get_utc_offset()

    def alter_table(table: str, column: str):
        conn.execute(
            sa.text(
                f"""
            ALTER TABLE {table}
            ALTER COLUMN {column} TYPE TIMESTAMP WITH TIME ZONE USING created_at AT TIME ZONE {utc_offset_str}
            """
            )
        )

    alter_table("trails", "created_at")
    alter_table("peeks", "created_at")
    alter_table("visits", "created_at")


def downgrade() -> None:
    conn = op.get_bind()

    def alter_table(table: str, column: str):
        conn.execute(
            sa.text(
                f"""
            ALTER TABLE {table}
            ALTER COLUMN {column} TYPE TIMESTAMP WITHOUT TIME ZONE USING created_at
            """
            )
        )

    alter_table("trails", "created_at")
    alter_table("peeks", "created_at")
    alter_table("visits", "created_at")
