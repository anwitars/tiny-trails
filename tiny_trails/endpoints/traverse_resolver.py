from fastapi import Request
from fastapi.exceptions import HTTPException
from starlette.responses import RedirectResponse

from tiny_trails.endpoints.common.models import Visit
from tiny_trails.endpoints.common.tools import get_ip_from_request, hash_ip

from .shorten_resolver import in_memory_trails


async def resolver(trail_id: str, request: Request):
    trail = in_memory_trails.get(trail_id)
    if trail is None:
        raise HTTPException(status_code=404, detail="Trail not found")

    if ip := get_ip_from_request(request):
        visit = Visit(hashed_ip=hash_ip(ip))
        in_memory_trails[trail_id].visits.append(visit)

    return RedirectResponse(trail.url, status_code=302)
