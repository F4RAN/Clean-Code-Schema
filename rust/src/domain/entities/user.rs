use super::super::value_objects::id::ID;
use super::super::value_objects::phone_number::PhoneNumber;
use super::super::value_objects::role::Role;
use super::super::value_objects::age::Age;
use super::super::value_objects::sex::Sex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)] 
pub struct User{
    pub id: ID,
    pub phone_number: PhoneNumber,
    pub role: Role,
    pub age: Age,
    pub sex: Sex
}