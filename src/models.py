from pydantic import BaseModel


class BaseResponse(BaseModel):
    status: bool


class UserAgentResponse(BaseResponse):
    ua: str


class ResponseError(BaseResponse):
    msg: str
