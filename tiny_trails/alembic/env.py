from logging.config import fileConfig

from alembic import context
from sqlalchemy import create_engine, pool

from tiny_trails.env import get_env
from tiny_trails.tables.base import Base

config = context.config

if config.config_file_name is not None:
    fileConfig(config.config_file_name)

target_metadata = Base.metadata


def _get_db_url() -> str:
    # NOTE: When running tests, we can not pass x arguments to alembic (or neither can we set env variables),
    # therefore using a tag argument to determine if we are running tests or not.
    # If it is set to "test", we will use the database URL from alembic.ini which gets overriden by the test setup.
    if context.get_tag_argument() == "test":
        url = config.get_main_option("sqlalchemy.url")
        assert (
            url is not None
        ), "sqlalchemy.url must be set in alembic.ini for test runs"
        return url

    return context.get_x_argument(as_dictionary=True).get("db_url", get_env("DB"))


def run_migrations_offline() -> None:
    """Run migrations in 'offline' mode.

    This configures the context with just a URL
    and not an Engine, though an Engine is acceptable
    here as well.  By skipping the Engine creation
    we don't even need a DBAPI to be available.

    Calls to context.execute() here emit the given string to the
    script output.

    """
    context.configure(
        url=_get_db_url(),
        target_metadata=target_metadata,
        literal_binds=True,
        dialect_opts={"paramstyle": "named"},
    )

    with context.begin_transaction():
        context.run_migrations()


def run_migrations_online() -> None:
    """Run migrations in 'online' mode.

    In this scenario we need to create an Engine
    and associate a connection with the context.

    """
    engine = create_engine(
        _get_db_url(),
        poolclass=pool.NullPool,
    )

    with engine.connect() as connection:
        context.configure(connection=connection, target_metadata=target_metadata)

        with context.begin_transaction():
            context.run_migrations()


if context.is_offline_mode():
    run_migrations_offline()
else:
    run_migrations_online()
