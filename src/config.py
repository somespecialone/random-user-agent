import os
from pathlib import Path

from pydantic import BaseSettings, AnyUrl, FilePath


class Settings(BaseSettings):
    PROJECT_TITLE: str = "Random User Agent"
    PROJECT_VERSION: str = "1.0.0"
    PROJECT_DESCRIPTION: str = "Simple 🚀 API that get random user-agent on each GET request!"

    PROJECT_TERMS_OF_SERVICE: AnyUrl = "https://github.com/somespecialone/random-user-agent/"

    PROJECT_CONTACT_NAME: str = "somespecialone"
    PROJECT_CONTACT_EMAIL: str = "tkachenkodmitriy@yahoo.com"

    PROJECT_LICENSE_NAME: str = "MIT"
    PROJECT_LICENSE_URL: AnyUrl = "https://github.com/somespecialone/random-user-agent/blob/master/LICENSE"

    USER_AGENTS_FILE: FilePath = Path(
        "/var/task/res/user_agents.txt" if os.getenv("DETA_RUNTIME") else "res/user_agents.txt"
    )


settings = Settings()
