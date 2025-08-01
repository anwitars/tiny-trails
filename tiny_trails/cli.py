#!/usr/bin/env python3

import click

from tiny_trails.env import ENVIRONMENT_PREFIX


@click.group()
def cli():
    pass


@cli.command()
@click.option("--host", default="127.0.0.1", help="Host to run the server on.")
@click.option("--port", default=3000, help="Port to run the server on.")
@click.option("--reload", default=False, help="Enable auto-reload for development.")
@click.option(
    "--db",
    required=True,
    help="Database URL for the application.",
    envvar=f"{ENVIRONMENT_PREFIX}DB",
    show_envvar=True,
)
def serve(host: str, port: int, reload: bool, db: str):
    """Start the server."""

    import uvicorn

    from tiny_trails.app import create_app

    uvicorn.run(
        create_app(db_url=db),
        host=host,
        port=port,
        reload=reload,
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
