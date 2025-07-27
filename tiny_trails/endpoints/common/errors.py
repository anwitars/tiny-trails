from fastapi import HTTPException


class TrailNotFoundOrExpiredError(HTTPException):
    def __init__(self) -> None:
        super().__init__(404, "Trail not found or expired")
