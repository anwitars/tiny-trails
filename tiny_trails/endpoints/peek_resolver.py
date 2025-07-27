from datetime import datetime

from tiny_trails.endpoints.common import TrailNotFoundOrExpiredError
from tiny_trails.endpoints.common.models import Peek

from .shorten_resolver import in_memory_trails


async def resolver(trail_id: str):
    now = datetime.now()
    trail = in_memory_trails.get(trail_id)

    if trail is None or trail.is_expired(reference=now):
        raise TrailNotFoundOrExpiredError()

    trail.peeks.append(Peek())

    return trail.url
