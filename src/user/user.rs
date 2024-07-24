use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserCreationResult {
    pub id: i32,
}

#[derive(Serialize)]
pub struct UserLoginToken {
    pub token: u32,
}
