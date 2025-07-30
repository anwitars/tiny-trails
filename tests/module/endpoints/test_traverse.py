from datetime import datetime

from freezegun import freeze_time
from httpx import AsyncClient

from tests.module.conftest import async_test
from tests.utils.scenario.scenario import Scenario
from tiny_trails.endpoints.common.tools import hash_ip


@async_test
async def test_ok(client: AsyncClient, scenario: Scenario):
    trail = scenario.add_trail()

    response = await client.get(f"/t/{trail.data.trail_id}", follow_redirects=False)

    assert response.status_code == 302
    assert response.headers["Location"] == trail.data.url

    visits = trail.visits()
    assert len(visits) == 1
    assert visits[0].hashed_ip == hash_ip("127.0.0.1")


@async_test
async def test_not_found(client: AsyncClient):
    # this is an invalid trail id, it will never exist
    response = await client.get("/t/0", follow_redirects=False)
    assert response.status_code == 404


@async_test
@freeze_time("2025-07-27")
async def test_expired(client: AsyncClient, scenario: Scenario):
    trail = scenario.add_trail(
        created_at=datetime(2025, 7, 20),
        lifetime=24 * 3,
    )

    response = await client.get(f"/t/{trail.data.trail_id}", follow_redirects=False)

    assert response.status_code == 404
