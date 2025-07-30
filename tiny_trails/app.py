"""
This file will be executed by the FastAPI server to start the application.
"""

from fastapi import FastAPI

from tiny_trails.database import PostgresDatabase
from tiny_trails.env import get_env
from tiny_trails.middlewares import create_middlewares
from tiny_trails.routing import assign_routes

db_url = get_env("DB")
db = PostgresDatabase(url=db_url)

app = FastAPI(middleware=create_middlewares(db))

assign_routes(app)
