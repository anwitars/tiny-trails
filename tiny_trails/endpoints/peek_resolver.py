from datetime import datetime

from fastapi import Request
from sqlalchemy import select

from tiny_trails.endpoints.common import TrailNotFoundOrExpiredError
from tiny_trails.middlewares.context import get_context_from_request


async def resolver(trail_id: str, request: Request):
    """
    Get a quick peek at a Trail's URL by its ID. This does not leave a Visit on the Trail,
    therefore not counting towards the Trail's visit statistics. Might be useful for
    automation.
    """

    from tiny_trails.tables import Peek, Trail

    context = get_context_from_request(request)
    now = datetime.now()
    async with context.db.session_scope() as session:
        trail = await session.execute(select(Trail).where(Trail.trail_id == trail_id))
        trail = trail.scalar_one_or_none()

        if trail is None or trail.is_expired(reference=now):
            raise TrailNotFoundOrExpiredError()

        session.add(Peek(trail_id=trail.id, created_at=now))

        await session.commit()

    return trail.url
