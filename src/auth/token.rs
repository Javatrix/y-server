use std::time::{Duration, Instant};

use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;

pub struct Token {
    value: String,
    owner: String,
    creation_time: Instant,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

impl Token {
    pub fn new(owner: String) -> Self {
        Token {
            value: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect(),
            owner,
            creation_time: Instant::now(),
        }
    }

    pub fn lifetime(&self) -> Duration {
        self.creation_time.elapsed()
    }

    pub fn get_value(&self) -> &String {
        return &self.value;
    }

    pub fn get_owner(&self) -> &String {
        return &self.owner;
    }
}
