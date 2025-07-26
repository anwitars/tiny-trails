from fastapi.testclient import TestClient
from mock import patch

from tiny_trails.endpoints.shorten_resolver import Trail, Visit


def test_ok(client: TestClient):
    trails = {
        "a": Trail(
            url="https://google.com",
            visits=[
                Visit(hashed_ip="a"),
                Visit(hashed_ip="a"),
                Visit(hashed_ip="b"),
                Visit(hashed_ip="c"),
            ],
        )
    }

    with patch("tiny_trails.endpoints.info_resolver.in_memory_trails", trails):
        response = client.get("/info/a", follow_redirects=False)

        assert response.status_code == 200

        data = response.json()
        assert data
        assert data["id"] == "a"
        assert data["url"] == "https://google.com"
        assert data["visits"]["all"] == 4
        assert data["visits"]["unique"] == 3
