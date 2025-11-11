import express from "express";
import { MongoUserRepository } from "./infrastructure/dbs/MongoUserRepository.js";
import { CreateUser } from "./application/usecases/CreateUser";
import { createUserController } from "./presentation/http/CreateUserController.js";
import { getMongoClient } from "./config/db.js";


async function start() {
    const mongo = await getMongoClient()
    const repo = new MongoUserRepository(mongo);
    const createUser = new CreateUser(repo);
    
    const app = express();
    app.use(express.json());
    app.post("/create_user", createUserController(createUser));
    app.listen(3000, () => {
        console.log("Server running on http://localhost:3000");
    });
}

start().catch(console.error);
