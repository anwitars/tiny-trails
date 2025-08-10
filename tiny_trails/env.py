ENVIRONMENT_PREFIX = "TINY_TRAILS_"
"""The application prefix."""

APPLICATION_PKG_NAME = "tiny_trails"


def get_pkg_file(path: str) -> str:
    from importlib.resources import files

    return str(files(APPLICATION_PKG_NAME).joinpath(path))


def get_env(key: str) -> str:
    """
    Get an environment variable with the application prefix.
    """

    from os import getenv

    key_with_prefix = f"{ENVIRONMENT_PREFIX}{key}"
    value = getenv(key_with_prefix)
    if not value:
        raise ValueError(f"Environment variable '{key_with_prefix}' is not set.")

    return value
