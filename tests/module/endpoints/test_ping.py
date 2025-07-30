from httpx import AsyncClient

from tests.module.conftest import async_test


@async_test
async def test_ping(client: AsyncClient):
    response = await client.get("/ping")
    assert response.status_code == 200
    assert response.text == "pong"
