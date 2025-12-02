struct Age{
    value: i32
}

impl Age{
    fn new(value: i32) -> Result<Age, String>{
        if(value > 120) {
            Err(String::from("Age can not be greater than 120"))
        }
        else if(value < 0){
            Err(String::from("Age can not smaller than 0"))
        }else{
            Ok(Age{value})
        }
    }
}