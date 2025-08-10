#!/usr/bin/env python3

from typing import cast

import click

from tiny_trails.env import ENVIRONMENT_PREFIX


@click.group()
def cli():
    pass


@cli.command()
@click.option(
    "--host",
    default="127.0.0.1",
    show_default=True,
    help="Host to run the server on.",
)
@click.option(
    "--port",
    default=3000,
    show_default=True,
    help="Port to run the server on.",
)
@click.option(
    "--reload",
    default=False,
    show_default=True,
    help="Enable auto-reload for development.",
)
@click.option(
    "--db",
    required=True,
    help="Database URL for the application.",
    envvar=f"{ENVIRONMENT_PREFIX}DB",
    show_envvar=True,
)
@click.option(
    "--apply-migrations",
    default=True,
    show_default=True,
    help="Apply database migrations on startup.",
    envvar=f"{ENVIRONMENT_PREFIX}APPLY_MIGRATIONS",
    show_envvar=True,
)
def serve(
    host: str,
    port: int,
    reload: bool,
    db: str,
    apply_migrations: bool,
):
    """Start the server."""

    from tiny_trails.logging import init as init_logging

    log_config = init_logging()

    import uvicorn

    from tiny_trails.app import create_app

    if apply_migrations:
        from alembic.command import upgrade
        from alembic.config import Config as AlembicConfig

        from tiny_trails.env import get_pkg_file

        alembic_cfg = AlembicConfig(get_pkg_file("alembic.ini"))
        alembic_cfg.set_main_option("sqlalchemy.url", db)
        alembic_cfg.set_section_option("logger_alembic", "level", "WARNING")
        upgrade(alembic_cfg, "heads", tag="cli")

    uvicorn.run(
        create_app(db_url=db),
        host=host,
        port=port,
        reload=reload,
        log_config=cast(dict, log_config),
    )


@cli.command()
@click.option(
    "--output",
    default="docs/openapi.json",
    help="Output file for OpenAPI docs.",
)
def generate_docs(output: str):
    """
    Generate OpenAPI documentation and save it to a file.
    """

    import json

    from fastapi.openapi.utils import get_openapi

    from tiny_trails.app import create_app_for_docs

    app = create_app_for_docs()

    with open(output, "w") as f:
        json.dump(
            get_openapi(
                title=app.title,
                version=app.version,
                description=app.description,
                routes=app.routes,
            ),
            f,
            indent=2,
        )


if __name__ == "__main__":
    cli()
