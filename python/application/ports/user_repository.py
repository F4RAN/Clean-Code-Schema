from domain.entities.user import User
from domain.value_objects.id import ID
from abc import ABC, abstractmethod


class UserRepository(ABC):
    @abstractmethod
    def find_by_id(self, id:ID):
        pass
    @abstractmethod
    def save(self, user:User):
        pass
        