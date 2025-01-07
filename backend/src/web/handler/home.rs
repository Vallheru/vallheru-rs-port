use crate::web::{AppState, Result};
use axum::{extract::State, Json};

use vallheru::api::{LoginRequest, LoginResponse};

pub async fn post_login(
    State(ctx): State<AppState>,
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
        crate::repository::player::alter_last_login(&ctx.db, player.id).await?;

        Ok(Json(LoginResponse {
            id: player.id,
            token,
            username: player.username,
        }))
    } else {
        Err(crate::web::Error::InternalServer(format!(
            "cannot get token for the player id: {}",
            player.id
        )))
    }
}
