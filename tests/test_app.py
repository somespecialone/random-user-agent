def test_main_route(client, agents):
    resp = client.get("/")

    assert resp.ok

    resp_json = resp.json()

    assert resp_json["ua"] in agents
