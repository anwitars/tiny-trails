from dataclasses import dataclass, field
from datetime import datetime
from string import ascii_lowercase, ascii_uppercase

from pydantic import BaseModel, Field, HttpUrl


@dataclass(frozen=True, eq=False)
class Visit:
    hashed_ip: str
    created: datetime = field(default_factory=datetime.now)


@dataclass
class Trail:
    url: str
    visits: list[Visit] = field(default_factory=list)
    created: datetime = field(default_factory=datetime.now)


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
    in_memory_trails[trail_id] = Trail(url=str(pave_input.url))

    return PaveResponse(
        trail_id=trail_id,
        message=f"Trail paved successfully with ID: {trail_id}",
    )
