from starlette.applications import Starlette
from starlette.responses import PlainTextResponse, Response
from starlette.routing import Route


async def ping() -> Response:
    return PlainTextResponse("pong")


app = Starlette(routes=[Route("/ping", ping)])
