use std::{collections::HashMap, sync::Mutex};

use anyhow::Error;

// pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

pub struct Data {
    tokens: Mutex<HashMap<u64, String>>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            tokens: Mutex::new(HashMap::new()),
        }
    }

    pub fn save_user_token(&self, user_id: u64, token: &str) {
        let mut tokens = self.tokens.lock().expect("Acquire api token lock");

        tokens.insert(user_id, token.to_string());
    }

    pub fn get_user_token(&self, user_id: u64) -> Option<String> {
        let tokens = self.tokens.lock().expect("Acquire api token lock");

        tokens.get(&user_id).map(|x| x.to_string())
    }
}
