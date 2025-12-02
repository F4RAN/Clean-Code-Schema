enum Sex{
    Male,
    Female
}

impl Sex{
    fn new(value: String) -> Result<Sex, String>{
        match value.to_lowercase(){
            "male" => Ok(Sex::Male),
            "female" => Ok(Sex::Female),
            _ => Err(String::from("Sex can be just male or female"))
        }
    }
}