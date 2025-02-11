use std::collections::HashMap;

use super::token::Token;

pub struct TokenManager {
    token_lifetime: u64,
    tokens: HashMap<String, Token>,
}

impl Default for TokenManager {
    fn default() -> Self {
        TokenManager::new(5 * 60)
    }
}

impl TokenManager {
    pub fn new(token_lifetime: u64) -> Self {
        Self {
            token_lifetime,
            tokens: Default::default(),
        }
    }

    pub fn create_token(&mut self, owner: &String) -> &Token {
        self.tokens.insert(owner.clone(), Token::new(owner.clone()));
        return self.get_token(owner).unwrap();
    }

    pub fn get_token(&self, owner: &String) -> Option<&Token> {
        self.tokens.get(owner)
    }

    pub fn owns_valid_token(&self, owner: &String) -> bool {
        let token = self.get_token(owner);
        token.is_some() && token.unwrap().lifetime().as_secs() <= self.token_lifetime
    }

    pub fn is_valid(&self, owner: &String, token_value: &String) -> bool {
        self.owns_valid_token(owner) && *self.get_token(owner).unwrap().get_value() == *token_value
    }
}
