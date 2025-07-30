from dataclasses import dataclass

from starlette.requests import Request
from starlette.types import ASGIApp, Receive, Scope, Send

from tiny_trails.database import Database

REQUEST_CONTEXT_SCOPE_KEY = "context"


@dataclass
class RequestContext:
    """
    Context that is available per-request.
    """

    db: Database


def get_context_from_request(request: Request) -> RequestContext:
    context: RequestContext | None = request.scope.get(REQUEST_CONTEXT_SCOPE_KEY)
    if context is None:
        raise RuntimeError(
            "Request context is not available. "
            "Ensure that the ContextMiddleware is applied to the application."
        )

    return context


class ContextMiddleware:
    f"""
    Middleware that provides a per-request RequestContext to the application.
    The context will be injected into the request scope under the key "{REQUEST_CONTEXT_SCOPE_KEY}".
    """

    app: ASGIApp
    db: Database

    def __init__(self, app: ASGIApp, db: Database) -> None:
        self.app = app
        self.db = db

    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        scope[REQUEST_CONTEXT_SCOPE_KEY] = RequestContext(
            db=self.db,
        )

        await self.app(scope, receive, send)
