from httpx import AsyncClient

from tests.module.conftest import async_test
from tests.utils.scenario.scenario import Scenario


@async_test
async def test_ok(client: AsyncClient, scenario: Scenario):
    trail = scenario.add_trail()

    response = await client.get(f"/peek/{trail.data.trail_id}")

    assert response.status_code == 200
    assert response.text == trail.data.url

    assert len(trail.peeks()) == 1
