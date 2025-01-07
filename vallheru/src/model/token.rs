use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Token {
    pub player_id: i32,
    pub token: String,
    pub created_at: crate::DateTime,
    pub valid_until: crate::DateTime,
}

pub fn generate_token(player_id: i32) -> String {
    format!(
        "{}{}{}",
        player_id,
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4()
    )
    .replace("-", "")
}

impl Token {
    pub fn new(player_id: i32) -> Self {
        Self {
            player_id,
            token: generate_token(player_id),
            created_at: chrono::Utc::now(),
            valid_until: chrono::Utc::now() + Duration::hours(12),
        }
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn generate_token() {
        assert_eq!(super::generate_token(12).len(), 66);
        assert_eq!(&super::generate_token(12)[0..2], "12");
        assert_eq!(&super::generate_token(1112)[0..4], "1112");
    }
}
