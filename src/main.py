import random
from typing import Union

from fastapi import FastAPI

from src.models import UserAgentResponse, ResponseError
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


@app.get("/", response_model=Union[UserAgentResponse, ResponseError])
def get_random_ua():
    try:
        with settings.USER_AGENTS_FILE.open("r") as ua:
            random_row = random.choice(ua.readlines())

        return {"status": True, "ua": random_row.replace("\n", "")}

    except Exception as e:
        return {"status": False, "msg": e.args[0]}
