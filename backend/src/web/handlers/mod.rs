use std::sync::Arc;

use crate::web::{ApiContext, Result};
use axum::{extract::State, Json};

use vallheru::api::{LoginRequest, LoginResponse};

pub async fn post_login(
    State(ctx): State<Arc<ApiContext>>,
    req: Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let player = crate::repository::player::get_user_by_email_and_password(
        &ctx.db,
        &req.email,
        &req.password,
    )
    .await?;

    let mut token = crate::repository::token::get_active_token_for_player(&ctx.db, player.id).await;
    if token.is_none() {
        token = crate::repository::token::create_token_for_player(&ctx.db, player.id).await;
    }
    if let Some(token) = token {
        Ok(Json(LoginResponse {
            id: player.id,
            token,
            username: player.username,
        }))
    } else {
        Err(crate::web::Error::InternalServerError(format!(
            "cannot get token for the player id: {}",
            player.id
        )))
    }
}
