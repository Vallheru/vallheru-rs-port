use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Store, Serialize, Deserialize)]
pub struct Context {
    pub auth: String,
    pub id: i32,
    pub in_game: bool,
}

pub const STORAGE_KEY: &str = "vallheru-player-context";

impl Context {
    pub fn new() -> Self {
        Self {
            auth: "".to_string(),
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
