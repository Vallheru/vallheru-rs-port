use crate::{
    repository::{
        player::{alter_last_login_and_login_count, get_player_by_email},
        token::{create_token_for_player, get_active_token_for_player, get_token},
    },
    web::handler::prelude::*,
};
use vallheru::{api::{IsTokenValidRequest, IsTokenValidResponse, LoginRequest, LoginResponse}, date};

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

pub async fn post_is_token_valid(
    State(ctx): State<AppState>,
    req: Json<IsTokenValidRequest>,
) -> Result<Json<IsTokenValidResponse>> {
    let token = get_token(&ctx.db, &req.token).await;

    Ok(match token {
        None => Json(IsTokenValidResponse{
            is_valid: false,
            reason: String::from("Token does not exist"),
        }),
        Some(token) => {
            let now = date::new_now();
            if token.valid_until < now {
                Json(IsTokenValidResponse{
                    is_valid: false,
                    reason: String::from("Token expired")
                })
            } else {
            Json(IsTokenValidResponse{
                    is_valid: true,
                    reason: String::from(""),
                })
            }
        }
    })
}
