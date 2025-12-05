use super::super::super::domain::entities::user::User;
use super::super::super::domain::value_objects::sex::Sex;
use super::super::super::domain::value_objects::age::Age;
use super::super::super::domain::value_objects::id::ID;
use super::super::super::domain::value_objects::phone_number::PhoneNumber;
use super::super::super::domain::value_objects::role::Role;
use super::super::ports::user_repository::UserRepository;


pub struct CreateUser{
    user_repo:  Box<dyn UserRepository + Send + Sync>
}
impl CreateUser{
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> CreateUser{
        CreateUser{
            user_repo
        }
    }
    pub async fn execute(&self, phone_number: PhoneNumber, role: Role, age: Age, sex: Sex ) -> Result<User, String>{
        let user = User{
            id: ID::new(),
            phone_number,
            role,
            age,
            sex
        };
        let stored_user = self.user_repo.save(user).await;
        Ok(stored_user.unwrap())
        
    }
}