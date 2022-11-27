import random

from fastapi import FastAPI

from src.models import UserAgentResponse
from src.config import settings


app = FastAPI(
    title=settings.PROJECT_TITLE,
    description=settings.PROJECT_DESCRIPTION,
    version=settings.PROJECT_VERSION,
    terms_of_service=settings.PROJECT_TERMS_OF_SERVICE,
    contact={
        "name": settings.PROJECT_CONTACT_NAME,
        "email": settings.PROJECT_CONTACT_EMAIL,
    },
    license_info={
        "name": settings.PROJECT_LICENSE_NAME,
        "url": settings.PROJECT_LICENSE_URL,
    },
)

with settings.USER_AGENTS_FILE.open("r") as ua:
    agents = tuple(ua.read().splitlines())


@app.get("/", response_model=UserAgentResponse)
def get_random_ua():
    return {"ua": random.choice(agents)}
