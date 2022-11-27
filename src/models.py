from pydantic import BaseModel


class UserAgentResponse(BaseModel):
    ua: str
