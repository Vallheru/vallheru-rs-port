use crate::{
    repository::{
        player::{alter_last_login_and_login_count, get_player_by_email},
        token::{create_token_for_player, get_active_token_for_player},
    },
    web::handler::prelude::*,
};
use vallheru::api::{LoginRequest, LoginResponse};

pub async fn post_login(
    State(ctx): State<AppState>,
    req: Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let player = get_player_by_email(&ctx.db, &req.email, &req.password).await?;

    let mut token = get_active_token_for_player(&ctx.db, player.id).await;

    if token.is_none() {
        token = create_token_for_player(&ctx.db, player.id).await;
    }

    match token {
        Some(token) => {
            alter_last_login_and_login_count(&ctx.db, player.id).await?;

            Ok(Json(LoginResponse {
                id: player.id,
                token,
                username: player.username,
                login_count: player.login_count,
            }))
        }
        None => Err(crate::web::Error::InternalServer(format!(
            "cannot get token for the player id: {}",
            player.id
        ))),
    }
}
