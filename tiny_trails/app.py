from fastapi import FastAPI

from tiny_trails.database import PostgresDatabase
from tiny_trails.middlewares import create_middlewares
from tiny_trails.routing import assign_routes


def _get_version() -> str:
    from importlib.metadata import version

    return version("tiny-trails")


def create_app(*, db_url: str) -> FastAPI:
    """
    Create a FastAPI application instance with middlewares and routes.
    """

    db = PostgresDatabase(url=db_url)
    app = FastAPI(middleware=create_middlewares(db), version=_get_version())
    assign_routes(app)

    return app


def create_app_for_docs() -> FastAPI:
    """
    Create a FastAPI application instance for generating OpenAPI documentation.
    This instance does not include middlewares or database connections.
    """

    app = FastAPI(version=_get_version())
    assign_routes(app)
    return app
