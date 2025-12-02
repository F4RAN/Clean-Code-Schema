from domain.entities.user import User
from application.usecases.create_user import CreateUser
from config.db import connect_mongodb
from infrastructure.dbs.mongo_user_repository import MongoUserRepository
from presentation.http.create_user_controller import create_user_controller
from fastapi import Request, FastAPI
app = FastAPI()       
mongo_client = connect_mongodb()
user_mongo_instance = MongoUserRepository(mongo_client)
create_user_usecase = CreateUser(user_mongo_instance)



app.post("/create_user")(create_user_controller(create_user_usecase))