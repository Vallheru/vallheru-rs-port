use crate::web::{AppState, Result};
use axum::{extract::State, Json};

use vallheru::api::{LoginRequest, LoginResponse};

pub async fn post_login(
    State(ctx): State<AppState>,
    req: Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let player =
        crate::repository::player::get_player_by_email(&ctx.db, &req.email, &req.password).await?;

    let mut token = crate::repository::token::get_active_token_for_player(&ctx.db, player.id).await;
    if token.is_none() {
        token = crate::repository::token::create_token_for_player(&ctx.db, player.id).await;
    }
    if let Some(token) = token {
        crate::repository::player::alter_last_login_and_login_count(&ctx.db, player.id).await?;

        Ok(Json(LoginResponse {
            id: player.id,
            token,
            username: player.username,
            login_count: player.login_count,
        }))
    } else {
        Err(crate::web::Error::InternalServer(format!(
            "cannot get token for the player id: {}",
            player.id
        )))
    }
}
