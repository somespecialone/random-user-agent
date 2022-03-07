def test_main_route(client):
    resp = client.get("/")

    assert resp.ok

    resp_json = resp.json()

    assert resp_json["status"]
    assert resp_json["ua"]
