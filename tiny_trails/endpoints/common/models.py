from dataclasses import dataclass, field
from datetime import datetime

from tiny_trails.endpoints.common.tools import generate_trail_token

Hours = int  # to make it clear

TRAIL_DEFAULT_LIFETIME: Hours = 24 * 3
TRAIL_MINIMUM_LIFETIME: Hours = 1
TRAIL_MAXIMUM_LIFETIME: Hours = 24 * 30

TRAIL_TOKEN_HEADER = "X-Trail-Token"


@dataclass(frozen=True, eq=False)
class Visit:
    hashed_ip: str
    created: datetime = field(default_factory=datetime.now)


@dataclass
class Trail:
    url: str
    visits: list[Visit] = field(default_factory=list)
    created: datetime = field(default_factory=datetime.now)
    lifetime: Hours = field(default=TRAIL_DEFAULT_LIFETIME)
    token: str = field(default_factory=generate_trail_token)

    def is_expired(self, reference: datetime | None = None) -> bool:
        if reference is None:
            reference = datetime.now()
        return (reference - self.created).total_seconds() > self.lifetime * 3600
