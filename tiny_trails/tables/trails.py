from datetime import datetime

from sqlalchemy import TIMESTAMP, VARCHAR, text
from sqlalchemy.orm import Mapped, mapped_column

from tiny_trails.endpoints.common.models import (
    TRAIL_DEFAULT_LIFETIME,
    TRAIL_TOKEN_LENGTH,
    Hours,
)
from tiny_trails.utils import utc_now

from .base import Base


def is_trail_expired(
    created_at: datetime,
    lifetime: Hours,
    reference: datetime | None = None,
) -> bool:
    return ((reference or utc_now()) - created_at).total_seconds() > lifetime * 3600


class Trail(Base):
    __tablename__ = "trails"

    url: Mapped[str]

    # we do not actually need to for identifying the trail, but it is more effective to
    # store a sequence number for each trail to generate trail_id via base52
    id: Mapped[int] = mapped_column(
        primary_key=True,
        autoincrement=True,
        default=None,  # autoincrement
    )
    trail_id: Mapped[str] = mapped_column(
        unique=True,
        index=True,
        default=None,  # trigger
    )
    token: Mapped[str] = mapped_column(
        VARCHAR(TRAIL_TOKEN_LENGTH),
        index=True,
        default=None,  # sql function
        server_default=text("generate_token()"),
    )
    lifetime: Mapped[int] = mapped_column(
        default=TRAIL_DEFAULT_LIFETIME,
        server_default=text(str(TRAIL_DEFAULT_LIFETIME)),
    )
    created_at: Mapped[datetime] = mapped_column(
        TIMESTAMP(timezone=True),
        default_factory=utc_now,
        server_default=text("now()"),
    )

    def is_expired(self, reference: datetime | None = None) -> bool:
        return is_trail_expired(self.created_at, self.lifetime, reference)
