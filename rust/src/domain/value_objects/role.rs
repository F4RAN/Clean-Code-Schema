use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub enum Role{
    Admin,
    User,
    Guest
}


impl Role{
    pub fn new(value: String) -> Result<Role,String>{
        match value.to_lowercase().as_str(){
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "guest" => Ok(Role::Guest),
            _ => Err(String::from("Role must be admin, user, or guest"))
        }
    }
}