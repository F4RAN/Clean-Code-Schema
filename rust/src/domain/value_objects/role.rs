enum Role{
    Admin,
    User,
    Guest
}


impl Role{
    fn new(value: String) -> Result<Role,String>{
        match value.to_lowercase(){
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "guest" => Ok(Role::Guest),
            _ => Err(String::from("Role must be admin, user, or guest"))
        }
    }
}