
from application.usecases.create_user import CreateUser
from fastapi import Request

def create_user_controller(create_user_usecase: CreateUser):
    async def handler(request: Request):
        body = await request.json()
        user = await create_user_usecase.execute(
            id=body.get("id"),
            phone_number=body.get("phone_number"),
            role=body.get("role"),
            age=body.get("age"),
            sex=body.get("sex")
        )
        return user
    return handler