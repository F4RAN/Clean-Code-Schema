export default class ID {
    private static holder: number = 0;
    
    constructor() {
        ID.holder++;
        this.value = ID.holder;
    }
    
    private value: number;
    
    getValue(): number {
        return this.value;
    }
}