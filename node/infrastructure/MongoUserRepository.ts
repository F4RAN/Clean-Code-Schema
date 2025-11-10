import { UserRepository } from "../application/ports/UserRepository";
import ID from "../domain/user/ID";
import User from "../domain/user/User";

export class MongoUserRepository implements UserRepository{
    constructor(private mongo){
        this.mongo = mongo
    }
    async save(user: User){
        await this.mongo.user.create(user)
    }
    async findById(id: ID): Promise<User | null> {
        return this.mongo.user.findById(id)
    }
}