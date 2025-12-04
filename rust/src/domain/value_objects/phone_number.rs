use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PhoneNumber{
    value: String
}

impl PhoneNumber{
    pub fn new(value: String) -> Result<PhoneNumber,String>{
        if(value.len() != 10){
            Err(String::from("Phone number must be 10 characters"))
        }
        else if(!value.chars().all(|c| c.is_ascii_digit())){
            Err(String::from("Phone number characters must be digit"))
        }
        else{
            Ok(PhoneNumber{value})
        }
    }
}