pub mod error;
use axum::Router;
pub use error::Error;
pub mod handler;
pub mod middleware;

mod router;
use handler::{method_not_allowed_fallback, not_found_fallback};
use router::api_router;

use anyhow::Context;
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};

use sqlx::PgPool;

use crate::config::Config;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}

pub type AppState = Arc<ApiContext>;

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .context("cannot bind network interface")?;

    let shared_state = Arc::new(ApiContext {
        config: Arc::new(config),
        db,
    });

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(Any)
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api_router(shared_state.clone()))
        .with_state(shared_state)
        .method_not_allowed_fallback(method_not_allowed_fallback)
        .fallback(not_found_fallback)
        .layer(cors);

    axum::serve(listener, app)
        .await
        .context("failed to serve axum app")
}
