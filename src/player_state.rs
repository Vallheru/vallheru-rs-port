use axum::extract::{FromRef, FromRequestParts};
use axum::response::Html;
use http::request::Parts;
use minijinja::{context, Environment, Value};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use crate::model::Player;

use crate::{
    repository::{player::get_player_by_token, token::extend_token_for_player}, 
    web::{middleware::AuthError, AppState, Error},
};


#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SessionData {
    pub token: String
}

impl SessionData {
    fn empty(&self) -> bool {
        self.token.is_empty()
    }
}

impl SessionData {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Debug)]
#[allow(dead_code)] // TODO: Fix it;
pub struct PlayerState {
    session: Session,
    pub player: Option<Player>,
    data: SessionData,
}

impl PlayerState {
    pub const SESSION_FIELD: &'static str = "VALLHERU_PLAYER_STATE";

    /// Function returns an ApiError derivable error if not logged in
    pub fn must_be_logged_in(&self) -> Result<(), AuthError> {
        match self.player{
            None => Err(AuthError::InvalidAuthorizationToken)?,
            Some(_) => Ok(())
        }
    }

    pub fn game_context(&self, ctx: Value) -> Value {
        context! { 
            ..ctx,
            ..context! {
                is_logged_in => self.must_be_logged_in().is_ok(),
                player => self.player
            }
        }
    }

    pub fn render_error(&self, tpl_env: &Environment<'static>, error_msg: &str, return_link: &str) -> Html<String> {
        let template = tpl_env.get_template("game_error.html").unwrap();
        let r = template
            .render(self.game_context(context! {
                error_msg => error_msg,
                return_link => return_link,
            }))
            .unwrap();
        
        Html(r)
    }
}

impl<S> FromRequestParts<S> for PlayerState
where 
    AppState: FromRef<S>,
    S: Send + Sync
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state)
            .await?;

        let session_data: SessionData = session
            .get(Self::SESSION_FIELD)
            .await?
            .unwrap_or_default();

        if session_data.empty() {
            return Ok(Self{
                session,
                player: None,
                data: session_data,
            })
        }
        let app_state = AppState::from_ref(state);

        let player = get_player_by_token(&app_state.db, &session_data.token).await;

        match  player {
            None => Err(AuthError::InvalidAuthorizationToken)?,
            Some(player) => {
                extend_token_for_player(&app_state.db, player.id, &session_data.token).await?;

                Ok(Self {
                    session,
                    player: Some(player),
                    data: session_data,
                })
            }
        }
    }
}
