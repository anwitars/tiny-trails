from fastapi import Request
from sqlalchemy import select
from starlette.responses import RedirectResponse

from tiny_trails.endpoints.common import (
    TrailNotFoundOrExpiredError,
    get_ip_from_request,
    hash_ip,
)
from tiny_trails.middlewares.context import get_context_from_request
from tiny_trails.utils import utc_now


async def resolver(trail_id: str, request: Request):
    """
    Traverse a Trail by its ID, leaving a Visit on the Trail. The user gets redirected to the Trail's URL.
    """

    from tiny_trails.tables import Trail, Visit

    context = get_context_from_request(request)
    now = utc_now()

    async with context.db.session_scope() as session:
        trail = await session.execute(select(Trail).where(Trail.trail_id == trail_id))
        trail = trail.scalar_one_or_none()

        if trail is None or trail.is_expired(reference=now):
            raise TrailNotFoundOrExpiredError()

        if ip := get_ip_from_request(request):
            visit = Visit(hashed_ip=hash_ip(ip), created_at=now, trail_id=trail.id)
            session.add(visit)

        trail_url = trail.url

        await session.commit()

    return RedirectResponse(trail_url, status_code=302)
