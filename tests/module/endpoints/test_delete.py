from httpx import AsyncClient

from tests.module.conftest import async_test
from tests.utils.scenario.scenario import Scenario
from tiny_trails.endpoints.common.models import TRAIL_TOKEN_HEADER


@async_test
async def test_ok(client: AsyncClient, scenario: Scenario):
    trail = scenario.add_trail(token="token")

    response = await client.delete(
        f"/t/{trail.data.trail_id}", headers={TRAIL_TOKEN_HEADER: "token"}
    )

    assert response.status_code == 204
    assert not trail.exists
