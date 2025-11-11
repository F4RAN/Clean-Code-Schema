import ID from "../../domain/user/value_objects/ID";
import User from "../../domain/user/entities/User";
import { UserRepository } from "../ports/UserRepository";

export class CreateUser{
    constructor(public userRepo: UserRepository){
        this.userRepo = userRepo
    }
    async execute({
        id,
        phoneNumber,
        role,
        age,
        sex
    }){
        const user = new User({
            id: new ID(),
            phoneNumber: phoneNumber,
            role: role,
            age: age,
            sex: sex
        })
        await this.userRepo.save(user)
        return user
    }
}