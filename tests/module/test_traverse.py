from fastapi.testclient import TestClient
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
