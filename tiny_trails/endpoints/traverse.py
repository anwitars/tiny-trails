from fastapi.exceptions import HTTPException
from starlette.responses import RedirectResponse

from .shorten import in_memory_trails


async def resolver(trail_id: str):
    if trail_id not in in_memory_trails:
        raise HTTPException(status_code=404, detail="Trail not found")

    url = in_memory_trails[trail_id]
    return RedirectResponse(url, status_code=302)
