ENVIRONMENT_PREFIX = "TINY_TRAILS_"
"""The application prefix."""


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
