use axum::{middleware::from_fn_with_state, routing::get, Router};
use tower_http::services::ServeDir;

use super::{
    handler::{
        home::{get_index, get_login, get_news, get_register, post_login},
        player_statistics::{
            get_disable_protection, get_player_statistics, get_select_bloodline, get_select_class,
            get_select_gender, get_select_religion, get_use_ap,
        },
    },
    middleware::{authorization_middleware, game_authorization_middleware},
    AppState,
};

pub fn static_router() -> Router<AppState> {
    let serve_public_dir = ServeDir::new("./public");

    Router::new().nest_service("/public", serve_public_dir)
}

pub fn home_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_index))
        .route("/login", get(get_login).post(post_login))
        .route("/register", get(get_register))
}

pub fn game_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/news", get(get_news))
        .nest("/player-statistics", player_statistics_route())
        .layer(from_fn_with_state(app_state, game_authorization_middleware))
}

pub fn player_statistics_route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_player_statistics))
        .route("/use-ap", get(get_use_ap))
        .route("/select-bloodline", get(get_select_bloodline))
        .route("/select-class", get(get_select_class))
        .route("/select-religion", get(get_select_religion))
        .route("/select-gender", get(get_select_gender))
        .route("/disable-protection", get(get_disable_protection))
}

pub fn api_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        // .route("/is-token-valid", post(home::post_is_token_valid))
        // .route("/login", post(home::post_login))
        .nest("/game", in_game_router(app_state))
}

fn in_game_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        // .route("/player/{identifier}", get(get_player))
        .layer(from_fn_with_state(app_state, authorization_middleware))
}
