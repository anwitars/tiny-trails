from fastapi.testclient import TestClient
from mock import patch

from tiny_trails.endpoints.shorten_resolver import Trail


def test_ok(client: TestClient):
    with patch(
        "tiny_trails.endpoints.peek_resolver.in_memory_trails", {}
    ) as mock_trails:
        trail = Trail(url="https://google.com/")
        mock_trails["a"] = trail

        response = client.get("/peek/a")
        assert response.status_code == 200
        assert response.text == trail.url

        assert len(trail.peeks) == 1
