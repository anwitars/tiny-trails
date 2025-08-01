from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from logging.config import _DictConfigArgs

DEFAULT_LOGGING_CONFIG: "_DictConfigArgs" = {
    "version": 1,
    "disable_existing_loggers": False,
    "formatters": {
        "default": {
            "format": "%(asctime)s - %(levelname)s - %(filename)s:%(lineno)d - %(name)s: %(message)s",
        }
    },
    "handlers": {
        "default": {
            "level": "INFO",
            "class": "logging.StreamHandler",
            "formatter": "default",
        }
    },
    "root": {
        "level": "INFO",
        "handlers": ["default"],
    },
    "loggers": {
        "sqlalchemy": {
            "propagate": True,
            "level": "WARNING",
            "handlers": ["default"],
        }
    },
}


def init(filename: str | None = None) -> "_DictConfigArgs":
    """
    Initialize logging configuration.

    If a filename is provided, it will load the configuration from that file.
    Otherwise, it uses the default logging configuration.
    Returns the logging configuration dictionary.
    """
    from logging.config import dictConfig

    if filename:
        import json

        import yaml

        with open(filename, "r") as file:
            if filename.endswith(".json"):
                config = json.load(file)
            elif filename.endswith(".yaml") or filename.endswith(".yml"):
                config = yaml.safe_load(file)
            else:
                raise ValueError("Unsupported configuration file format.")
    else:
        config = DEFAULT_LOGGING_CONFIG

    dictConfig(config)
    return config
