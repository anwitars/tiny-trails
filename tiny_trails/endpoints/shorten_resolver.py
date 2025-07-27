from dataclasses import dataclass
from string import ascii_lowercase, ascii_uppercase

from pydantic import BaseModel, Field, HttpUrl

from tiny_trails.endpoints.common import (
    TRAIL_DEFAULT_LIFETIME,
    TRAIL_MAXIMUM_LIFETIME,
    TRAIL_MINIMUM_LIFETIME,
    Hours,
    Trail,
)

in_memory_trails: dict[str, Trail] = {}
TRAIL_ID_ALPHABET = ascii_lowercase + ascii_uppercase


def encode_base52(num: int) -> str:
    if num == 0:
        return TRAIL_ID_ALPHABET[0]

    base52 = []

    while num > 0:
        num, remainder = divmod(num, 52)
        base52.append(TRAIL_ID_ALPHABET[remainder])

    return "".join(reversed(base52))


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
    trail_id: str
    message: str


async def resolver(pave_input: PaveInput) -> PaveResponse:
    """
    Paves a Trail for the given URL.
    """

    trail_sequence_id = len(in_memory_trails)
    trail_id = encode_base52(trail_sequence_id)
    in_memory_trails[trail_id] = Trail(
        url=str(pave_input.url),
        lifetime=pave_input.lifetime,
    )

    return PaveResponse(
        trail_id=trail_id,
        message=f"Trail paved successfully with ID: {trail_id}",
    )
