pub mod error;
use axum::{response::Html, Router};
pub use error::Error;
pub mod handler;
pub mod middleware;
pub mod form;

mod api;

mod router;
use handler::{method_not_allowed_fallback, not_found_fallback};
use minijinja::{Environment, Value};
use router::{api_router, game_router, home_router, static_router};

use anyhow::Context;
use tower_sessions_sqlx_store::PostgresStore;
use crate::util::number::to_ordinal;
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};
use tower_sessions::SessionManagerLayer;

use sqlx::PgPool;

use crate::config::Config;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
    pub tpl_env: Environment<'static>,
}

impl ApiContext {
    pub fn render(&self, tpl_name: &str, ctx: Value) -> Html<String> {
        let template = self.tpl_env.get_template(tpl_name).unwrap();
        let r = template
            .render(ctx)
            .unwrap();
        Html(r)
    }
}

pub type AppState = Arc<ApiContext>;


pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let mut env = minijinja::Environment::new();
    env.add_function("to_ordinal", to_ordinal);

    minijinja_embed::load_templates!(&mut env);

    let session_store = PostgresStore::new(db.clone());
    session_store.migrate().await?;
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false);

    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .context("cannot bind network interface")?;

    let shared_state = Arc::new(ApiContext {
        config: Arc::new(config),
        db,
        tpl_env: env,
    });

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(Any)
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(static_router())
        .merge(home_router())
        .nest("/game", game_router(shared_state.clone()))
        .nest("/api", api_router(shared_state.clone()))
        .with_state(shared_state)
        .method_not_allowed_fallback(method_not_allowed_fallback)
        .fallback(not_found_fallback)
        .layer(cors)
        .layer(session_layer);

    axum::serve(listener, app)
        .await
        .context("failed to serve axum app")
}
