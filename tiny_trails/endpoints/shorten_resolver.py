from dataclasses import dataclass

from fastapi import Request
from pydantic import BaseModel, Field, HttpUrl

from tiny_trails.endpoints.common import (
    TRAIL_DEFAULT_LIFETIME,
    TRAIL_MAXIMUM_LIFETIME,
    TRAIL_MINIMUM_LIFETIME,
    Hours,
)
from tiny_trails.middlewares.context import get_context_from_request


class PaveInput(BaseModel):
    url: HttpUrl = Field(description="The URL to pave a Trail for.")
    lifetime: Hours = Field(
        default=TRAIL_DEFAULT_LIFETIME,
        description=f"The lifetime of the Trail in hours. Defaults to {TRAIL_DEFAULT_LIFETIME} hours.",
        ge=TRAIL_MINIMUM_LIFETIME,
        le=TRAIL_MAXIMUM_LIFETIME,
    )


@dataclass
class PaveResponse:
    trail_id: str = Field(description="The unique identifier for the paved Trail.")
    token: str = Field(
        description="The unique token for the Trail. This grants access to restricted operations."
    )
    message: str = Field(
        description="A message indicating the result of the operation."
    )


async def resolver(pave_input: PaveInput, request: Request) -> PaveResponse:
    """
    Paves a Trail for the given URL.
    """
    from tiny_trails.tables import Trail

    context = get_context_from_request(request)

    async with context.db.session_scope() as session:
        trail = Trail(
            url=str(pave_input.url),
            lifetime=pave_input.lifetime,
        )
        session.add(trail)
        await session.flush()
        await session.refresh(trail)

        trail_id, trail_token = trail.trail_id, trail.token

        await session.commit()

    return PaveResponse(
        trail_id=trail_id,
        message=f"Trail paved successfully with ID: {trail_id}",
        token=trail_token,
    )
