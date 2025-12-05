
const phoneNumberRegex = /^\d+$/;
export default class PhoneNumber{
    readonly value: string;
    constructor(value: string) {
        if (typeof value !== "string") 
            throw new Error("Phone number must be string");
        if (!phoneNumberRegex.test(value))
            throw new Error("Phone number must contain only digits");
        if (value.length !== 10) 
            throw new Error("Phone number length must be 10");
        this.value = value
    }
}