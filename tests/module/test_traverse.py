from fastapi.testclient import TestClient

from tiny_trails.endpoints.shorten_resolver import Trail, in_memory_trails


def test_ok(client: TestClient):
    in_memory_trails["a"] = Trail(url="https://google.com")

    response = client.get("/t/a", follow_redirects=False)

    assert response.status_code == 302
    assert response.headers["Location"] == "https://google.com"
