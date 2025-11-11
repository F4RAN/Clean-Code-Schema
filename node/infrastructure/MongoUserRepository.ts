import { Db } from "mongodb";
import { UserRepository } from "../application/ports/UserRepository";
import ID from "../domain/user/value_objects/ID";
import User from "../domain/user/entities/User";

export class MongoUserRepository implements UserRepository{
    private collection: any;
    constructor(public db: Db){
        this.db = db
        this.collection = db.collection('users')
    }
    async save(user: User): Promise<void>{
        await this.collection.insertOne(user)
    }
    async findById(id: ID): Promise<User | null> {
        return this.collection.findOne(id.getValue())
    }
}