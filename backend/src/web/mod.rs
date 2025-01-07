pub mod error;
pub use error::Error;

pub mod handlers;

use anyhow::Context;
use std::sync::Arc;

use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::config::Config;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .context("cannot bind network interface")?;

    let shared_state = Arc::new(ApiContext {
        config: Arc::new(config),
        db,
    });
    let app = Router::new()
        .nest("/api", api_router())
        .with_state(shared_state);

    axum::serve(listener, app)
        .await
        .context("failed to serve axum app")
}

fn api_router() -> Router<Arc<ApiContext>> {
    Router::new().route("/login", post(crate::web::handlers::post_login))
}
