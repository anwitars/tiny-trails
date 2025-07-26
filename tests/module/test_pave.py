from fastapi.testclient import TestClient
from mock import patch


def test_ok(client: TestClient):
    with patch(
        "tiny_trails.endpoints.shorten_resolver.in_memory_trails", {}
    ) as mock_trails:
        response = client.post("/pave", json={"url": "https://google.com"})

        assert response.status_code == 200
        assert len(mock_trails) == 1

        data = response.json()
        assert data
        assert data["trail_id"] == "a"
