from hashlib import sha256

from fastapi import Request
from fastapi.exceptions import HTTPException
from starlette.responses import RedirectResponse

from .shorten_resolver import Visit, in_memory_trails


def hash_ip(ip: str) -> str:
    return sha256(ip.encode()).hexdigest()


async def resolver(trail_id: str, request: Request):
    trail = in_memory_trails.get(trail_id)
    if trail is None:
        raise HTTPException(status_code=404, detail="Trail not found")

    if client := request.client:
        visit = Visit(hashed_ip=hash_ip(client.host))
        in_memory_trails[trail_id].visits.append(visit)

    return RedirectResponse(trail.url, status_code=302)
