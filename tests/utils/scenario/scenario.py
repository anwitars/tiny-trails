from datetime import datetime

from sqlalchemy.orm import Session

from tests.utils.scenario.base import ScenarioSessionScopeMixin
from tests.utils.scenario.trail_scenario import ScenarioTrail
from tests.utils.string import random_string
from tests.utils.sync_db import SyncDatabase
from tiny_trails.endpoints.common.models import (
    TRAIL_MAXIMUM_LIFETIME,
    TRAIL_MINIMUM_LIFETIME,
)
from tiny_trails.tables.trails import Trail


class _Unset:
    pass


_UNSET = _Unset()


class Scenario(ScenarioSessionScopeMixin):
    """
    A database-scoped scenario.
    A 'scenario' is an object that handles the creation and management of database objects
    while providing some useful utilities for testing.
    """

    db: SyncDatabase

    def __init__(self, db: SyncDatabase) -> None:
        self.db = db

    def add_trail(
        self,
        *,
        url: str | None = None,
        token: str | None = None,
        created_at: datetime | None = None,
        lifetime: int | None | _Unset = _UNSET,
        session: Session | None = None,
    ) -> ScenarioTrail:
        data = Trail(
            url=url or random_string(20),
        )

        if token:
            data.token = token

        if created_at:
            data.created_at = created_at

        if lifetime is not None:
            if isinstance(lifetime, _Unset):
                from random import randint

                lifetime = randint(
                    TRAIL_MINIMUM_LIFETIME,
                    TRAIL_MAXIMUM_LIFETIME,
                )

            data.lifetime = lifetime

        with self.session_scope(session) as session:
            return ScenarioTrail(
                data,
                self.db,
                session=session,
            )
