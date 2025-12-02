from application.ports.user_repository import UserRepository
from domain.entities.user import User
from domain.value_objects.id import ID
from domain.value_objects.age import Age
from domain.value_objects.phone_number import PhoneNumber
from domain.value_objects.role import Role
from domain.value_objects.sex import Sex


class CreateUser:
    def __init__(self, user_repo: UserRepository) -> None:
        self.user_repo = user_repo
    
    async def execute(self, id, phone_number, role, age, sex):
        user = User(uid=ID(), phone_number=PhoneNumber(phone_number), role=Role(role), age=Age(age), sex=Sex(sex))
        await self.user_repo.save(user)
        return user