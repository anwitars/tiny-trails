from typing import AsyncIterator

from fastapi import FastAPI
from httpx import ASGITransport, AsyncClient
from pytest import fixture, mark
from pytest_asyncio import fixture as async_fixture

from tests.utils.async_db import AsyncDatabase
from tests.utils.scenario.scenario import Scenario
from tests.utils.string import random_string
from tests.utils.sync_db import SyncDatabase
from tiny_trails.env import get_env, get_pkg_file
from tiny_trails.middlewares import create_middlewares
from tiny_trails.middlewares.context import Database

async_test = mark.asyncio


@fixture(scope="session")
def admin_db_conn():
    """
    Provides a connection to the admin database.
    """

    from psycopg2 import connect

    url = get_env("TEST_ADMIN_DB")
    conn = connect(url)
    yield conn
    conn.close()


@fixture(scope="session")
def test_db_url():
    """
    Creates a session-scoped temporary database and yields its URL.

    I have been experimenting with single-session db connections, without any commits,
    which is possible if I did not need alembic upgrades. But since important triggers
    and functions are created during the upgrade, I need to run them somehow.
    So this fixture creates a database, runs the migrations,
    and then drops the database after the tests are done.
    """

    from alembic import command
    from alembic.config import Config
    from psycopg2 import connect
    from psycopg2.extensions import ISOLATION_LEVEL_AUTOCOMMIT
    from psycopg2.sql import SQL, Identifier

    # even though there is a 'admin_db_conn' fixture, we need the url itself as well,
    # and it is cleaner to handle this way
    url = get_env("TEST_ADMIN_DB")
    base_url = url.rsplit("/", 1)[0]  # Get the base URL without the database name

    db_name = random_string()
    conn = connect(url)
    conn.set_isolation_level(ISOLATION_LEVEL_AUTOCOMMIT)

    with conn.cursor() as cursor:
        cursor.execute(SQL("CREATE DATABASE {}").format(Identifier(db_name)))

    test_url = f"{base_url}/{db_name}"

    config = Config(get_pkg_file("alembic.ini"))
    config.set_main_option("sqlalchemy.url", test_url)
    command.upgrade(config, "heads", tag="test")

    yield test_url

    with conn.cursor() as cursor:
        cursor.execute(
            SQL(
                """
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = %s
                AND pid <> pg_backend_pid();
            """
            ),
            (db_name,),
        )
        cursor.execute(SQL("DROP DATABASE IF EXISTS {}").format(Identifier(db_name)))


@fixture(scope="session")
def async_db(test_db_url: str) -> AsyncDatabase:
    return AsyncDatabase(test_db_url)


@fixture(scope="session")
def db(test_db_url: str) -> SyncDatabase:
    return SyncDatabase(test_db_url)


@async_fixture(scope="session")
async def client(async_db: Database) -> AsyncIterator[AsyncClient]:
    from tiny_trails.routing import assign_routes

    app = FastAPI(middleware=create_middlewares(async_db))
    assign_routes(app)

    async with AsyncClient(
        transport=ASGITransport(app), base_url="http://test"
    ) as client:
        yield client


@fixture(scope="function")
def scenario(db: SyncDatabase) -> Scenario:
    return Scenario(db)
