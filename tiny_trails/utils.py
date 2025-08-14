from datetime import UTC, datetime


def utc_now() -> datetime:
    return datetime.now(tz=UTC)


def get_trails_version() -> str:
    from importlib.metadata import version

    return version("tiny-trails")
