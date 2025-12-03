use super::super::domain::entities::user::User;
use super::super::domain::value_objects::id::ID;
pub trait UserRepository{
    async fn save(&self, user:User) -> Result<User,String>;
    async fn find_by_id(&self, id: ID) -> Result<User,String>;
}