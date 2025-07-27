from datetime import datetime

from fastapi import Request

from tiny_trails.endpoints.common import TrailNotFoundOrExpiredError
from tiny_trails.endpoints.common.models import TRAIL_TOKEN_HEADER

from .shorten_resolver import in_memory_trails


async def resolver(trail_id: str, request: Request):
    now = datetime.now()
    trail = in_memory_trails.get(trail_id)
    token = request.headers.get(TRAIL_TOKEN_HEADER)

    if (
        trail is None
        or trail.is_expired(reference=now)
        or (token and token != trail.token)
    ):
        raise TrailNotFoundOrExpiredError()

    del in_memory_trails[trail_id]
