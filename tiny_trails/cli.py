#!/usr/bin/env python3

import click


@click.group()
def cli():
    pass


@cli.command()
@click.option("--host", default="127.0.0.1", help="Host to run the server on.")
@click.option("--port", default=3000, help="Port to run the server on.")
@click.option("--prod", default=False, is_flag=True, help="Run in production mode.")
@click.option("--reload", default=False, help="Enable auto-reload for development.")
@click.option("--file", default=None, help="Path to the file to serve.")
def serve(host: str, port: int, prod: bool, reload: bool, file: str | None):
    """Start the server."""

    import subprocess

    fastapi_cmd = "run" if prod else "dev"

    optional_args: list[str] = []

    if reload:
        optional_args.append("--reload")

    # Here you would typically start your server, e.g.:
    subprocess.run(
        [
            "fastapi",
            fastapi_cmd,
            "--host",
            host,
            "--port",
            str(port),
            file or "tiny_trails/app.py",
            *optional_args,
        ]
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

    from tiny_trails.app import app

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
