from starlette.middleware import Middleware
from starlette.middleware.cors import CORSMiddleware

from tiny_trails.database import Database
from tiny_trails.middlewares.context import ContextMiddleware


def create_middlewares(db: Database) -> list[Middleware]:
    return [
        # as it is an open API, we allow all origins, methods, and headers
        Middleware(
            CORSMiddleware,
            allow_origins=["*"],  # Allow all origins for CORS
            allow_credentials=True,
            allow_methods=["*"],  # Allow all methods
            allow_headers=["*"],
        ),  # Allow all headers
        Middleware(ContextMiddleware, db=db),
    ]
