use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    body: String,
}

impl Post {
    pub(crate) fn new(title: String, body: String) -> Self {
        Self { title, body }
    }
}

