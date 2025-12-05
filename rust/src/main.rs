mod application;
mod domain;
mod presentation;
mod infrastracture;
mod config;
use config::db;
use tokio;
use axum::{Router, routing::post};
use crate::{application::usecases::create_user::CreateUser, infrastracture::dbs::mongo_user_repository::MongoUserRepository};
use presentation::http::create_user_controller::create_user_controller;

#[tokio::main]
async fn main() {
    let mongo_client = db::connect_db().await;
    let repo = MongoUserRepository::new(mongo_client);
    let create_user_usecase = CreateUser::new(Box::new(repo));
    
    let app: Router<_> = Router::new()
        .route("/create_user", post(create_user_controller(create_user_usecase)));
        // Additional routes
        // .route("/users", get(get_all_users_handler))
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}