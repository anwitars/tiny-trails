from typing import Sequence

from sqlalchemy import select
from sqlalchemy.orm import Session

from tests.utils.string import random_string
from tiny_trails.tables import Peek, Trail, Visit

from .base import ScenarioObject


class ScenarioTrail(ScenarioObject[Trail]):
    # TODO: this is probably over-engineering
    @property
    def exists(self) -> bool:
        return (
            self.from_select(select(Trail).where(Trail.id == self.data.id), self.db)
            is not None
        )

    def peeks(self, *, session: Session | None = None) -> Sequence[Peek]:
        with self.session_scope(session) as session:
            return (
                session.execute(select(Peek).where(Peek.trail_id == self.data.id))
                .scalars()
                .all()
            )

    def visits(
        self,
        *,
        session: Session | None = None,
    ) -> Sequence[Visit]:
        with self.session_scope(session) as session:
            visits = (
                session.execute(select(Visit).where(Visit.trail_id == self.data.id))
                .scalars()
                .all()
            )

            session.expunge_all()

            return visits

    def add_visit(
        self,
        ip: str | None = None,
        *,
        session: Session | None = None,
    ) -> Visit:
        with self.session_scope(session) as session:
            visit = Visit(trail_id=self.data.id, hashed_ip=ip or random_string(20))
            session.add(visit)
            session.flush()
            session.expunge(visit)

        return visit

    def add_peek(
        self,
        *,
        session: Session | None = None,
    ) -> Peek:
        with self.session_scope(session) as session:
            peek = Peek(trail_id=self.data.id)
            session.add(peek)
            session.flush()
            session.expunge(peek)

        return peek
