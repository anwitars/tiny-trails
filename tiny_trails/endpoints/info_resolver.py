from fastapi import Request
from pydantic import BaseModel, Field
from sqlalchemy import distinct, func, select

from tiny_trails.endpoints.common.errors import TrailNotFoundOrExpiredError
from tiny_trails.middlewares.context import get_context_from_request


class TrailVisitInfo(BaseModel):
    all: int = Field(
        description="The total number of visits to the Trail, including both unique and non-unique visits."
    )
    unique: int = Field(
        description="The number of unique visits to the Trail, based on distinct hashed IP addresses."
    )


class TrailInfo(BaseModel):
    id: str = Field(description="The unique identifier of the Trail.")
    url: str = Field(description="The URL associated with the Trail.")
    visits: TrailVisitInfo = Field(
        description="Information about visits to the Trail, including total and unique visits."
    )
    created: str = Field(
        description="The UTC timestamp when the Trail was created, in ISO 8601 format."
    )
    lifetime: int = Field(description="The lifetime of the Trail in hours.")


async def resolver(trail_id: str, request: Request) -> TrailInfo:
    """
    Retrieve information about a trail by its ID. See return schema for details.
    """

    from tiny_trails.tables import Trail, Visit
    from tiny_trails.tables.trails import is_trail_expired

    context = get_context_from_request(request)
    async with context.db.session_scope() as session:
        dbres = await session.execute(
            select(
                Trail.created_at,
                Trail.lifetime,
                Trail.url,
                func.count(Visit.id),
                func.count(distinct(Visit.hashed_ip)),
            )
            .where(Trail.trail_id == trail_id)
            .join(Visit, Visit.trail_id == Trail.id, isouter=True)
            .group_by(Trail.created_at, Trail.lifetime, Trail.url)
        )
        dbres = dbres.t.one_or_none()

        if dbres is None:
            raise TrailNotFoundOrExpiredError()

        created_at, lifetime, url, all_visits, unique_visits = dbres

        if is_trail_expired(created_at, lifetime):
            raise TrailNotFoundOrExpiredError()

        return TrailInfo(
            id=trail_id,
            url=url,
            visits=TrailVisitInfo(all=all_visits, unique=unique_visits),
            created=created_at.isoformat(),
            lifetime=lifetime,
        )
