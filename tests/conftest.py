import pytest
from fastapi.testclient import TestClient


@pytest.fixture(scope="session")
def client():
    from src import main

    with TestClient(main.app) as test_client:
        yield test_client
