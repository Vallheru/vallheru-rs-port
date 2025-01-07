use std::sync::Arc;

use crate::repository::player::get_player_by_token;
use crate::web::AppState;
use crate::web::{Error, Result};
use axum::{
    body::Body,
    extract::{Request, State},
    http,
    middleware::Next,
    response::Response,
};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("an authorization token is missing")]
    MissingAuthorizationToken,

    #[error("invalid an authorization token provided")]
    InvalidAuthorizationToken,
}

pub async fn authorization_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, Error> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| AuthError::InvalidAuthorizationToken),
        None => Err(AuthError::MissingAuthorizationToken),
    }?;

    let mut header = auth_header.split_whitespace();
    let (_token_type, token) = (header.next(), header.next());

    if let Some(token) = token {
        let player = get_player_by_token(&app_state.db, token).await;
        if let Some(player) = player {
            req.extensions_mut().insert(Arc::new(player));

            return Ok(next.run(req).await);
        }
    }

    Err(AuthError::InvalidAuthorizationToken)?
}
