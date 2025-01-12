use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub trait ApiToken {
    fn get_token(&self) -> String;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub token: String,
    pub id: i32,
    pub in_game: bool,
}

impl ApiToken for Context {
    fn get_token(&self) -> String {
        self.token.clone()
    }
}

pub const STORAGE_KEY: &str = "vallheru-player-context";

impl Context {
    pub fn new() -> Self {
        Self {
            token: String::from(""),
            id: 0,
            in_game: false,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| {
                storage
                    .get_item(STORAGE_KEY)
                    .ok()
                    .flatten()
                    .and_then(|val| serde_json::from_str::<Context>(&val).ok())
            })
            .unwrap_or(Self::new())
    }
}
