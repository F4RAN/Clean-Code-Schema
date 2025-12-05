import ID from '../value_objects/ID'
import Sex from '../value_objects/Sex'
import PhoneNumber from '../value_objects/PhoneNumber'
import Role from '../value_objects/Role'
import Age from '../value_objects/Age'
export default class User {
    id: ID;
    phoneNumber: PhoneNumber;
    sex: Sex;
    role: Role;
    age: Age;
    constructor({id, phoneNumber, sex, role, age}){
        if(!id) throw new Error("User must have id")
        this.id = id
        this.phoneNumber = phoneNumber
        this.sex = sex
        this.role = role
        this.age = age
        
    }
}