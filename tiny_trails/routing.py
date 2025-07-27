from fastapi import FastAPI
from starlette.responses import PlainTextResponse, RedirectResponse


def assign_routes(app: FastAPI):
    from tiny_trails.endpoints import delete, info, pave, peek, ping, traverse

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

    app.add_api_route(
        "/info/{trail_id}",
        info,
        methods=["GET"],
        summary="Get Trail Information",
    )

    app.add_api_route(
        "/t/{trail_id}",
        delete,
        methods=["DELETE"],
        summary="Delete Trail by ID",
        status_code=204,
    )

    app.add_api_route(
        "/peek/{trail_id}",
        peek,
        methods=["GET"],
        summary="Peek a Trail by ID",
        response_class=PlainTextResponse,
    )
