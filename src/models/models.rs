use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)] 
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}