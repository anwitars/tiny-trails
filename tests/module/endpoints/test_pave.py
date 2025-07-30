from httpx import AsyncClient
from sqlalchemy import select

from tests.module.conftest import async_test
from tests.utils.scenario.scenario import Scenario
from tests.utils.scenario.trail_scenario import ScenarioTrail
from tiny_trails.endpoints.common import TRAIL_MAXIMUM_LIFETIME, TRAIL_MINIMUM_LIFETIME
from tiny_trails.tables import Trail


def _assert_error(response, expected_error_type: str):
    assert response.status_code == 422
    data = response.json()
    assert data
    assert data["detail"][0]["type"] == expected_error_type


@async_test
async def test_ok(client: AsyncClient, scenario: Scenario):
    url = "https://testpave.com/"
    response = await client.post(
        "/pave",
        json={"url": url, "lifetime": 24},
    )

    assert response.status_code == 200
    data = response.json()
    assert data

    response_trail_id = data.get("trail_id")
    assert response_trail_id

    trail = ScenarioTrail.from_select(
        select(Trail).where(Trail.trail_id == response_trail_id),
        scenario.db,
    )
    assert trail

    assert trail.data.url == url
    assert trail.data.lifetime == 24
    assert trail.data.token == data["token"]


@async_test
async def test_validation(client: AsyncClient):
    response = await client.post(
        "/pave",
        json={"url": "google.com"},
    )
    _assert_error(response, "url_parsing")

    response = await client.post(
        "/pave",
        json={"url": "https://google.com/", "lifetime": TRAIL_MINIMUM_LIFETIME - 1},
    )
    _assert_error(response, "greater_than_equal")

    response = await client.post(
        "/pave",
        json={"url": "https://google.com/", "lifetime": TRAIL_MAXIMUM_LIFETIME + 1},
    )
    _assert_error(response, "less_than_equal")
