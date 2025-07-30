from typing import AsyncContextManager, Protocol

from sqlalchemy.ext.asyncio import (
    AsyncEngine,
    AsyncSession,
    async_sessionmaker,
    create_async_engine,
)


class Database(Protocol):
    """
    Protocol for a database session manager.
    Maybe later multiple databases will be supported.
    """

    def session_scope(self) -> AsyncContextManager[AsyncSession]: ...


class PostgresDatabase(Database):
    engine: AsyncEngine
    session_maker: async_sessionmaker[AsyncSession]

    def __init__(self, url: str) -> None:
        if not url.startswith("postgresql+asyncpg"):
            url = url.replace("postgresql", "postgresql+asyncpg")

        self.engine = self._create_engine(url)

        self.session_maker = async_sessionmaker(
            bind=self.engine,
            expire_on_commit=False,
        )

    # for easier test override
    @staticmethod
    def _create_engine(url: str) -> AsyncEngine:
        return create_async_engine(
            url,
            echo=True,
            future=True,
        )

    def session_scope(self) -> AsyncContextManager[AsyncSession]:
        return self.session_maker()
