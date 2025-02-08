use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use crate::model::Player;

use crate::web::Error;
use crate::{player_state::{PlayerState, SessionData}, repository::{player::{alter_last_login_and_login_count, get_player_by_email}, token::{create_token_for_player, get_active_token_for_player}}};

#[derive(thiserror::Error, Serialize, Deserialize, Debug)]
pub enum LoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("The database error")]
    SqlError,
    #[error("Failed to get or create session token")]
    TokenCreationError,
    #[error("Session setup failed")]
    SessionSetupError,
}

impl From<sqlx::Error> for LoginError {
    fn from(value: sqlx::Error) -> Self {
        error!("login_controller: {value:?}");
        Self::SqlError
    }
}

impl From<tower_sessions::session::Error> for LoginError {
    fn from(value: tower_sessions::session::Error) -> Self {
        error!("login_controller: {value:?}");
        Self::SessionSetupError
    }
}

pub type LoginResult<T> = Result<T, LoginError>;

pub async fn login(session: Session,
    db: &PgPool,
    email: &str,
    password: &str,
) -> LoginResult<Player> {
    let player = get_player_by_email(db, email, password)
        .await;


    if let Err(e) = player {
        println!("{:?}", e);

        return Err(LoginError::InvalidCredentials);
    }

    let player = player.unwrap();
        // .map_err(|_| )?; // TODO: make sure this is not sqlx error: (Error::Unauthorized)

    let mut token = get_active_token_for_player(db, player.id)
        .await;
    
    if token.is_none() {
        token = create_token_for_player(db, player.id).await;
    }

    match token {
        None => Err(LoginError::TokenCreationError),
        Some(token) => {
            alter_last_login_and_login_count(db, player.id).await?;
            session.insert(PlayerState::SESSION_FIELD, SessionData::new(token)).await?;

            Ok(player)
        }
    }
}
