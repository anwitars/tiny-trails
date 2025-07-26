from fastapi import FastAPI
from starlette.responses import PlainTextResponse, RedirectResponse

from tiny_trails.endpoints import pave, ping, traverse

app = FastAPI()


app.add_api_route(
    "/ping",
    ping,
    methods=["GET"],
    response_class=PlainTextResponse,
    summary="Responds with 'pong'",
)

app.add_api_route("/pave", pave, methods=["POST"], summary="Pave Trail")

app.add_api_route(
    "/t/{trail_id}",
    traverse,
    methods=["GET"],
    summary="Traverse Trail by ID",
    response_class=RedirectResponse,
)
