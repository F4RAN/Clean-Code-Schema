use super::super::domain::entities::user::User;
use super::super::domain::value_objects::sex::Sex;
use super::super::domain::value_objects::age::Age;
use super::super::domain::value_objects::id::ID;
use super::super::domain::value_objects::phone_number::PhoneNumber;
use super::super::domain::value_objects::role::Role;
use super::ports::user_repository::UserRepository;


pub struct CreateUser{
    user_repo: Box<dyn UserRepository>
}
impl CreateUser{
    fn new(user_repo: Box<dyn UserRepository>) -> CreateUser{
        CreateUser{
            user_repo
        }
    }
    fn execute(&self, phone_number: PhoneNumber, role: Role, age: Age, sex: Sex ) -> Result<User, String>{
        let user = User{
            id: ID::new(),
            phone_number,
            role,
            age,
            sex
        };
        let storedUser = self.user_repo.save(user)?;
        Ok(storedUser)
        
    }
}