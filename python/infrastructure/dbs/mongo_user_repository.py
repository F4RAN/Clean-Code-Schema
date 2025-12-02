
from application.ports.user_repository import UserRepository
from domain.value_objects.id import ID
from infrastructure.utils.decorators import sync_to_async



class MongoUserRepository(UserRepository):
    def __init__(self, db) -> None:
        super().__init__()
        self.collection = db["users"]
    
    @sync_to_async
    def save(self, user):
        self.collection.insert_one({
            "id": user.id._value,
            "phone_number": user.phone_number._value,
            "age": user.age._value,
            "sex": user.sex._value_,
            "role": user.role._value_
        })
    @sync_to_async
    def find_by_id(self, id: ID):
        return self.collection.find_one({"id": id.value})


        