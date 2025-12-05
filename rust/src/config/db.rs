
use mongodb::Client;

pub async fn connect_db() -> Client{
    let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "mongodb://127.0.0.1:27017/Mini-CC".to_string()
    });
    let client = Client::with_uri_str(db_connection_str).await.unwrap();
    client
}