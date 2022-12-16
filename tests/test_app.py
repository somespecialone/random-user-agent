def test_main_route(client, agents):
    resp = client.get("/")
    assert resp.status_code == 200
    resp_json = resp.json()
    assert resp_json["ua"] in agents


def test_get_all(client, agents):
    resp = client.get("/all")
    assert resp.status_code == 200
    resp_json: list[str] = resp.json()
    assert set(resp_json) == set(agents)
