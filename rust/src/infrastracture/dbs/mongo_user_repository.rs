use super::super::application::ports::user_repository::UserRepository;
use super::super::domain::entities::user::User;
use super::super::domain::value_objects::id::ID;
use mongodb::{Client, Collection};
use mongodb::bson::Document;
use mongodb::bson::doc;

pub struct MongoUserRepository{
    collection: Collection<Document>
}

impl MongoUserRepository{
    pub fn new(client: Client) -> MongoUserRepository{
        let my_coll: Collection<Document> = client
        .database("Mini-CC")
        .collection("users");
        return MongoUserRepository{ 
            collection: my_coll
        }
    }
}
impl UserRepository for MongoUserRepository {
    async fn save(&self, user:User) -> Result<User, String>{
        let doc = mongodb::bson::to_document(&user).map_err(|e| e.to_string())?;
        self.collection.insert_one(doc)
        .map_err(|e| e.to_string())?;
        Ok(user)
    } 
    async fn find_by_id(&self, id: ID) -> Result<User,String>{
        let doc_option = self.collection.find_one(doc!{"id": id.value()})
        .map_err(|e| e.to_string())?;
        match doc_option {
            Some(d) => mongodb::bson::from_document(d).map_err(|e| e.to_string()),
            None => Err("User not found".to_string())
        }
    }

}