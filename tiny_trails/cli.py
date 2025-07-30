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


if __name__ == "__main__":
    cli()
