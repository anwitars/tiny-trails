from datetime import datetime

from fastapi.testclient import TestClient
from freezegun import freeze_time
from mock import patch
from starlette.datastructures import Address

from tiny_trails.endpoints.common.tools import hash_ip
from tiny_trails.endpoints.shorten_resolver import Trail, in_memory_trails


def test_ok(client: TestClient):
    in_memory_trails["a"] = Trail(url="https://google.com")

    with patch.object(Address, "host", "127.0.0.1"):
        response = client.get("/t/a", follow_redirects=False)

    assert response.status_code == 302
    assert response.headers["Location"] == "https://google.com"

    visits = in_memory_trails["a"].visits
    assert len(visits) == 1

    # this will fail if we somehow forgot to hash the IP
    assert visits[0].hashed_ip == hash_ip("127.0.0.1")


def test_not_found(client: TestClient):
    with patch("tiny_trails.endpoints.traverse_resolver.in_memory_trails", {}):
        response = client.get("/t/a")
    assert response.status_code == 404


@freeze_time("2025-07-27")
def test_expired(client: TestClient):
    trail = Trail(url="", created=datetime(2025, 7, 20), lifetime=24 * 3)
    with patch(
        "tiny_trails.endpoints.traverse_resolver.in_memory_trails",
        {"a": trail},
    ):
        response = client.get("/t/a")
    assert response.status_code == 404
