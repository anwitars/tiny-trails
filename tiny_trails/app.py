from fastapi import FastAPI

from tiny_trails.routing import assign_routes

app = FastAPI()

assign_routes(app)
