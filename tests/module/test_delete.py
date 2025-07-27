from fastapi.testclient import TestClient
from mock import patch

from tiny_trails.endpoints.common.models import TRAIL_TOKEN_HEADER
from tiny_trails.endpoints.shorten_resolver import Trail


def test_ok(client: TestClient):
    with patch(
        "tiny_trails.endpoints.delete_resolver.in_memory_trails",
        {},
    ) as mocked_trails:
        mocked_trails["a"] = Trail(url="https://google.com", token="token")
        response = client.delete("/t/a", headers={TRAIL_TOKEN_HEADER: "token"})

        assert response.status_code == 204
        assert mocked_trails.get("a") is None


def test_token_mismatch(client: TestClient):
    with patch(
        "tiny_trails.endpoints.delete_resolver.in_memory_trails",
        {},
    ) as mocked_trails:
        mocked_trails["a"] = Trail(url="https://google.com", token="token")
        response = client.delete("/t/a", headers={TRAIL_TOKEN_HEADER: "invalid"})

        assert response.status_code == 404
