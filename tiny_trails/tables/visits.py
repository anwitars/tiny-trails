from datetime import datetime
from typing import TYPE_CHECKING

from sqlalchemy import ForeignKey, text
from sqlalchemy.orm import Mapped, mapped_column, relationship

from .base import Base

if TYPE_CHECKING:
    from .trails import Trail


class Visit(Base):
    __tablename__ = "visits"

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True, init=False)
    trail_id: Mapped[int] = mapped_column(ForeignKey("trails.id", ondelete="CASCADE"))
    hashed_ip: Mapped[str]
    created_at: Mapped[datetime] = mapped_column(
        default_factory=datetime.now,
        server_default=text("now()"),
    )

    trail: Mapped["Trail"] = relationship(
        init=False, primaryjoin="Visit.trail_id == Trail.id"
    )
