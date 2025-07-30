from contextlib import contextmanager
from typing import Generic, Self, TypeVar

from sqlalchemy import Select
from sqlalchemy.orm import Session

from tests.utils.sync_db import SyncDatabase
from tiny_trails.tables.base import Base as TableBase

T = TypeVar("T", bound=TableBase)

Sessionable = Session | SyncDatabase


class ScenarioSessionScopeMixin:
    db: SyncDatabase

    @contextmanager
    def session_scope(self, session: Sessionable | None = None):
        if session is not None:
            if isinstance(session, SyncDatabase):
                session = session.session_scope()
            yield session
        else:
            with self.db.session_scope() as session:
                yield session
                session.commit()


class ScenarioObject(Generic[T], ScenarioSessionScopeMixin):
    """
    An object that represents a row-scoped scenario in the database.
    """

    db: SyncDatabase
    data: T

    def __init__(
        self,
        data: T,
        db: SyncDatabase,
        *,
        session: Session | None = None,
    ) -> None:
        self.data = data
        self.db = db

        with self.session_scope(session) as session:
            if data not in session:
                session.add(data)
                session.flush()

            session.refresh(self.data)
            session.expunge(data)

    def refresh(self, session: Session | None = None) -> Self:
        with self.session_scope(session) as session:
            session.refresh(self.data)

        return self

    @classmethod
    def from_select(cls, select: Select[tuple[T]], db: SyncDatabase) -> Self | None:
        with db.session_scope() as session:
            data = session.execute(select).scalar_one_or_none()
            if data is None:
                return None

            return cls(data, db, session=session)
