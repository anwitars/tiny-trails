from starlette.responses import PlainTextResponse


async def resolver():
    """
    Simply returns a plain text response with the text "pong".
    """

    return PlainTextResponse("pong")
