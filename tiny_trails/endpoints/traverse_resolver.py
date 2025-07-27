from datetime import datetime

from fastapi import Request
from starlette.responses import RedirectResponse

from tiny_trails.endpoints.common import (
    TrailNotFoundOrExpiredError,
    Visit,
    get_ip_from_request,
    hash_ip,
)

from .shorten_resolver import in_memory_trails


async def resolver(trail_id: str, request: Request):
    now = datetime.now()
    trail = in_memory_trails.get(trail_id)

    if trail is None or trail.is_expired(reference=now):
        raise TrailNotFoundOrExpiredError()

    if ip := get_ip_from_request(request):
        visit = Visit(hashed_ip=hash_ip(ip), created=now)
        in_memory_trails[trail_id].visits.append(visit)

    return RedirectResponse(trail.url, status_code=302)
