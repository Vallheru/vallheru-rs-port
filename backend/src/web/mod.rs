pub mod error;
pub use error::Error;

pub mod handler;

pub mod middleware;

use anyhow::Context;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::config::Config;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}

type AppState = Arc<ApiContext>;

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .context("cannot bind network interface")?;

    let shared_state = Arc::new(ApiContext {
        config: Arc::new(config),
        db,
    });
    let app = Router::new()
        .nest("/api", api_router(shared_state.clone()))
        .with_state(shared_state)
        .fallback(handler::not_found_fallback);

    axum::serve(listener, app)
        .await
        .context("failed to serve axum app")
}

fn api_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", post(crate::web::handler::home::post_login))
        .route(
            "/game",
            get(protected).layer(axum::middleware::from_fn_with_state(
                app_state,
                middleware::authorization_middleware,
            )),
        )
}

async fn protected() -> String {
    String::from("hello")
}
