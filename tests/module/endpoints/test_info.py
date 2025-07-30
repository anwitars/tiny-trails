from httpx import AsyncClient

from tests.module.conftest import async_test
from tests.utils.scenario.scenario import Scenario


@async_test
async def test_ok(client: AsyncClient, scenario: Scenario):
    with scenario.db.session_scope() as session:
        trail = scenario.add_trail(session=session)
        trail.add_visit(ip="a", session=session)
        trail.add_visit(ip="a", session=session)
        trail.add_visit(ip="b", session=session)
        trail.add_visit(ip="c", session=session)
        session.commit()

    response = await client.get(f"/info/{trail.data.trail_id}", follow_redirects=False)

    assert response.status_code == 200

    data = response.json()
    assert data
    assert data["id"] == trail.data.trail_id
    assert data["url"] == trail.data.url
    assert data["visits"]["all"] == 4
    assert data["visits"]["unique"] == 3
