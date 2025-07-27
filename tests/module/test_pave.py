from fastapi.testclient import TestClient
from mock import patch

from tiny_trails.endpoints.common.models import (
    TRAIL_MAXIMUM_LIFETIME,
    TRAIL_MINIMUM_LIFETIME,
)


def _assert_error(response, expected_error_type: str):
    assert response.status_code == 422
    data = response.json()
    assert data
    assert data["detail"][0]["type"] == expected_error_type


def test_ok(client: TestClient):
    with patch(
        "tiny_trails.endpoints.shorten_resolver.in_memory_trails", {}
    ) as mock_trails:
        response = client.post(
            "/pave", json={"url": "https://google.com/", "lifetime": 24}
        )

        assert response.status_code == 200
        assert len(mock_trails) == 1

        data = response.json()
        assert data
        assert data["trail_id"] == "a"

        trail = mock_trails["a"]
        assert trail.url == "https://google.com/"
        assert trail.lifetime == 24


def test_validation(client: TestClient):
    response = client.post(
        "/pave",
        json={"url": "google.com"},
    )
    _assert_error(response, "url_parsing")

    response = client.post(
        "/pave",
        json={"url": "https://google.com/", "lifetime": TRAIL_MINIMUM_LIFETIME - 1},
    )
    _assert_error(response, "greater_than_equal")

    response = client.post(
        "/pave",
        json={"url": "https://google.com/", "lifetime": TRAIL_MAXIMUM_LIFETIME + 1},
    )
    _assert_error(response, "less_than_equal")
