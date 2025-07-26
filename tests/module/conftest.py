from fastapi import FastAPI
from fastapi.testclient import TestClient
from pytest import fixture


@fixture
def client() -> TestClient:
    from tiny_trails.routing import assign_routes

    app = FastAPI()
    assign_routes(app)

    return TestClient(app)
