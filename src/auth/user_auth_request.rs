use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserAuthRequest {
    pub username: String,
    pub token: String,
}
