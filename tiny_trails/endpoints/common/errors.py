from fastapi import HTTPException


class TrailNotFoundOrExpiredError(HTTPException):
    """
    Error returned from the API when a trail is not found or has expired.
    """

    def __init__(self) -> None:
        super().__init__(404, "Trail not found or expired")
