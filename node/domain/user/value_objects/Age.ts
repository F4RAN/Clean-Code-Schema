export default class Age{
    readonly value: number;
    constructor(value: number) {
        if (!Number.isInteger(value)) {
            throw new Error("Age must be an integer")
        }
        if(value < 0 || value > 120){
            throw new Error("Age must be between 0 and 120");
        }
        this.value = value
    }
}
