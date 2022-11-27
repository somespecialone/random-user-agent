import pytest
from fastapi.testclient import TestClient

from src import main, config


@pytest.fixture(scope="session")
def client():
    with TestClient(main.app) as test_client:
        yield test_client


@pytest.fixture(scope="session")
def agents():
    with config.settings.USER_AGENTS_FILE.open("r") as ua:
        yield ua.read().splitlines()
