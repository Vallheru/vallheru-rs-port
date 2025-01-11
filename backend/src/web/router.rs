use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};

use crate::web::handler::player::get_player;

use super::middleware::authorization_middleware;
use super::AppState;

pub fn api_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", post(crate::web::handler::home::post_login))
        .nest("/game", in_game_router(app_state))
}

fn in_game_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/player/{identifier}", get(get_player))
        .layer(from_fn_with_state(app_state, authorization_middleware))
}
