from dataclasses import dataclass, field
from datetime import datetime


@dataclass(frozen=True, eq=False)
class Visit:
    hashed_ip: str
    created: datetime = field(default_factory=datetime.now)


@dataclass
class Trail:
    url: str
    visits: list[Visit] = field(default_factory=list)
    created: datetime = field(default_factory=datetime.now)
