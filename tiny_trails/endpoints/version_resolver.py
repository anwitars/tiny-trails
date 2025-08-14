from starlette.responses import PlainTextResponse

from tiny_trails.utils import get_trails_version


async def resolver():
    """
    Returns a plain text response with the API version.
    """

    return PlainTextResponse(get_trails_version())
