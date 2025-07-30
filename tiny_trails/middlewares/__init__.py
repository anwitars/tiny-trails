from starlette.middleware import Middleware

from tiny_trails.database import Database
from tiny_trails.middlewares.context import ContextMiddleware


def create_middlewares(db: Database) -> list[Middleware]:
    return [Middleware(ContextMiddleware, db=db)]
