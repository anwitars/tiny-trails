from datetime import UTC, datetime
from typing import TYPE_CHECKING

from sqlalchemy import TIMESTAMP, ForeignKey, text
from sqlalchemy.orm import Mapped, mapped_column, relationship

from tiny_trails.utils import utc_now

from .base import Base

if TYPE_CHECKING:
    from .trails import Trail


class Visit(Base):
    __tablename__ = "visits"

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True, init=False)
    trail_id: Mapped[int] = mapped_column(ForeignKey("trails.id", ondelete="CASCADE"))
    hashed_ip: Mapped[str]
    created_at: Mapped[datetime] = mapped_column(
        TIMESTAMP(timezone=True),
        default_factory=utc_now,
        server_default=text("now()"),
    )

    trail: Mapped["Trail"] = relationship(
        init=False, primaryjoin="Visit.trail_id == Trail.id"
    )
