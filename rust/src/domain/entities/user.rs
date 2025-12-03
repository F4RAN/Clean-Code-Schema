use super::value_objects::id::ID;
use super::value_objects::phone_number::PhoneNumber;
use super::value_objects::role::Role;
use super::value_objects::age::Age;
use super::value_objects::sex::Sex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)] 
pub struct User{
    id: ID,
    phone_number: PhoneNumber,
    role: Role,
    age: Age,
    sex: Sex
}