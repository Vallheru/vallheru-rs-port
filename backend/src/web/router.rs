use axum::{
    middleware::from_fn_with_state,
    routing::{get, post, Route},
    Router,
};
use tower_http::services::ServeDir;

use super::middleware::authorization_middleware;
use super::AppState;
use crate::web::handler::home;
use crate::web::handler::player::get_player;

pub fn static_router() -> Router<AppState> {
    let serve_public_dir = ServeDir::new("./public");

    Router::new().nest_service("/public", serve_public_dir)
}

pub fn api_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/is-token-valid", post(home::post_is_token_valid))
        .route("/login", post(home::post_login))
        .nest("/game", in_game_router(app_state))
}

fn in_game_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/player/{identifier}", get(get_player))
        .layer(from_fn_with_state(app_state, authorization_middleware))
}
