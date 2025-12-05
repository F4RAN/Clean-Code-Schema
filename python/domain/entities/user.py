from ..value_objects.age import Age
from ..value_objects.id import ID
from ..value_objects.phone_number import PhoneNumber
from ..value_objects.sex import Sex
from ..value_objects.role import Role


class User:
    def __init__(self, uid: ID, phone_number: PhoneNumber, sex: Sex, role: Role, age: Age):
        self.id: ID = uid
        self.phone_number: PhoneNumber = phone_number
        self.sex: Sex = sex
        self.role: Role = role
        self.age: Age = age


