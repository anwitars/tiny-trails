from datetime import datetime

from fastapi import Request
from sqlalchemy import select

from tiny_trails.endpoints.common import TrailNotFoundOrExpiredError
from tiny_trails.endpoints.common.models import TRAIL_TOKEN_HEADER
from tiny_trails.middlewares.context import get_context_from_request


async def resolver(trail_id: str, request: Request):
    f"""
    Delete a trail by its ID. Must also provide the Trail's token in the '{TRAIL_TOKEN_HEADER}' header
    to authorize this action.
    """

    from tiny_trails.tables import Trail

    context = get_context_from_request(request)
    now = datetime.now()

    async with context.db.session_scope() as session:
        trail = await session.execute(select(Trail).where(Trail.trail_id == trail_id))
        trail = trail.scalar_one_or_none()
        token = request.headers.get(TRAIL_TOKEN_HEADER)

        if (
            trail is None
            or trail.is_expired(reference=now)
            or (token and token != trail.token)
        ):
            raise TrailNotFoundOrExpiredError()

        await session.delete(trail)
        await session.commit()
