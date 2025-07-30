from sqlalchemy import NullPool
from sqlalchemy.ext.asyncio import AsyncEngine, create_async_engine

from tiny_trails.database import PostgresDatabase


class AsyncDatabase(PostgresDatabase):
    @staticmethod
    def _create_engine(url: str) -> AsyncEngine:
        return create_async_engine(
            url,
            echo=True,
            future=True,
            poolclass=NullPool,  # to disable connection pooling for async operations
        )
