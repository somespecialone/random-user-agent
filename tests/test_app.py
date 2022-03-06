def test_main_route(client):
    resp = client.get("/").json()

    assert resp
