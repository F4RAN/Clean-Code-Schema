use super::super::super::application::usecases::create_user::CreateUser;
use super::super::super::domain::value_objects::phone_number::PhoneNumber;
use super::super::super::domain::value_objects::role::Role;
use super::super::super::domain::value_objects::age::Age;
use super::super::super::domain::value_objects::sex::Sex;
use super::super::super::domain::entities::user::User;
use serde::Deserialize;
use std::sync::Arc;

use axum::{
    Json,
    http::StatusCode,
};

#[derive(Deserialize)]
pub struct CreateUserRequest{
    phone_number: String,
    role: String,
    age: i32,
    sex: String
}

/**
 * This is a higher-order function (function that returns a function)
 * 
 * STEP 1: You call this with useCase (setup time)
 *   let handler = create_user_controller(use_case);
 *   - At this point: the HTTP request body DOESN'T EXIST YET
 *   - We're just creating the handler function
 *   - Returns: a closure (the inner function) that Axum will call
 * 
 * STEP 2: Axum calls the returned handler (request time)
 *   router.post("/users", handler);
 *   - When POST /users is hit, Axum automatically calls: handler(Json(body))
 *   - At this point: the request body is created by Axum and passed in
 *   - The closure captures `create_user` from STEP 1
 */
pub fn create_user_controller(create_user: CreateUser) -> impl Fn(Json<CreateUserRequest>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Json<User>, StatusCode>> + Send>> + Clone {
    // Wrap CreateUser in Arc (Atomic Reference Counted) so it can be shared
    // This allows the closure to be called multiple times
    let create_user = Arc::new(create_user);
    
    // This closure is what Axum will call when a request comes in
    // Axum provides the Json(body) when it calls this function
    move |Json(body): Json<CreateUserRequest>| {
        // Clone the Arc pointer (cheap operation, doesn't clone the actual data)
        // This allows us to move it into the async block
        let create_user = Arc::clone(&create_user);
        
        // Box::pin creates a heap-allocated, pinned future
        // This is required because async blocks have an unknown size at compile time
        Box::pin(async move {
            // Validate and convert the request body into value objects
            // If validation fails, return BAD_REQUEST status
            let phone_number = PhoneNumber::new(body.phone_number)
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            let role = Role::new(body.role)
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            let age = Age::new(body.age)
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            let sex = Sex::new(body.sex)
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            
            // Execute the use case (this is async, so we await it)
            // If execution fails, return INTERNAL_SERVER_ERROR status
            let user = create_user.execute(phone_number, role, age, sex)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            // Return the created user as JSON with OK status
            Ok(Json(user))
        })
    }
}