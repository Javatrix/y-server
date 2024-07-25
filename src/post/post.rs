use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub author_id: i32,
}

#[derive(Deserialize)]
pub struct PostCreationPayload {
    pub title: String,
    pub body: String,
    pub author_id: i32,
    pub token: String,
}
