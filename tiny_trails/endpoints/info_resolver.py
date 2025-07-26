from dataclasses import dataclass

from fastapi.exceptions import HTTPException

from .shorten_resolver import in_memory_trails


@dataclass(frozen=True, eq=False)
class TrailVisitInfo:
    all: int
    unique: int


@dataclass(frozen=True, eq=False)
class TrailInfo:
    id: str
    url: str
    visits: TrailVisitInfo
    created: str


async def resolver(trail_id: str) -> TrailInfo:
    trail = in_memory_trails.get(trail_id)
    if trail is None:
        raise HTTPException(404, "Trail is not found")

    all_visits = len(trail.visits)
    unique_visits = len(set(visit.hashed_ip for visit in trail.visits))

    return TrailInfo(
        id=trail_id,
        url=trail.url,
        visits=TrailVisitInfo(all=all_visits, unique=unique_visits),
        created=trail.created.isoformat(),
    )
