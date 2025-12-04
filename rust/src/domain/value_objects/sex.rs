use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub enum Sex{
    Male,
    Female
}

impl Sex{
    pub fn new(value: String) -> Result<Sex, String>{
        match value.to_lowercase().as_str(){
            "male" => Ok(Sex::Male),
            "female" => Ok(Sex::Female),
            _ => Err(String::from("Sex can be just male or female"))
        }
    }
}