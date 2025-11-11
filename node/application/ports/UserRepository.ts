import User from "../../domain/user/entities/User";
import ID from "../../domain/user/value_objects/ID";

export interface UserRepository{
    save(user: User): Promise<void>;
    findById(id: ID): Promise<User | null>
}
