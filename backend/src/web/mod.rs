pub mod error;
use axum::{extract::{FromRef, FromRequestParts}, response::Html, Router};
use http::request::Parts;
pub use error::Error;
pub mod handler;
pub mod middleware;
pub mod form;

mod router;
use handler::{method_not_allowed_fallback, not_found_fallback, player};
use middleware::AuthError;
use minijinja::{context, Environment, Value};
use router::{api_router, game_router, static_router};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tower_sessions_sqlx_store::PostgresStore;
use vallheru::{api::ApiError, model::Player, utils::to_ordinal};
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};
use tower_sessions::{Session, SessionManagerLayer};

use sqlx::PgPool;

use crate::{config::Config, repository::{player::get_player_by_token, token::extend_token_for_player}};
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

#[derive(Serialize, Deserialize, Default, Clone)]
struct SessionData {
    token: String
}

impl SessionData {
    fn empty(&self) -> bool {
        self.token.is_empty()
    }
}
pub struct InGameState {
    session: Session,
    player: Option<Player>,
    data: SessionData,
}

impl InGameState {
    const SESSION_FIELD: &'static str = "VALLHERU_IN_GAME_DATA";

    /// Function returns an ApiError derivable error if not logged in
    pub fn must_be_logged_in(&self) -> Result<()> {
        match self.player{
            None => Err(AuthError::InvalidAuthorizationToken)?,
            Some(_) => Ok(())
        }
    }

    /// Function called during logging in
    pub async fn log_in(&mut self, token: String, player: Player) -> Result<()> {
        self.data.token = token;
        self.player = Some(player);

        self.session.insert(Self::SESSION_FIELD, self.data.clone()).await?;

        Ok(())
    }

    pub fn game_context(&self) -> Value {
        context! {
            is_logged_in => self.must_be_logged_in().is_ok(),
            player => self.player
        }
    }
}

impl<S> FromRequestParts<S> for InGameState 
where 
    AppState: FromRef<S>,
    S: Send + Sync
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state)
            .await?;

        let session_data: SessionData = session
            .get(Self::SESSION_FIELD)
            .await?
            .unwrap_or_default();

        if session_data.empty() {
            return Ok(Self{
                session,
                player: None,
                data: session_data,
            })
        }

        let app_state = AppState::from_ref(state);

        let player = get_player_by_token(&app_state.db, &session_data.token).await;

        match  player {
            None => Err(AuthError::InvalidAuthorizationToken)?,
            Some(player) => {
                extend_token_for_player(&app_state.db, player.id, &session_data.token).await?;

                Ok(Self {
                    session,
                    player: Some(player),
                    data: session_data,
                })
            }
        }
    }
}



pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let mut env = minijinja::Environment::new();
    env.add_function("to_ordinal", to_ordinal);

    minijinja_embed::load_templates!(&mut env);

    let session_store = PostgresStore::new(db.clone());
    session_store.migrate().await?;
    let session_layer = SessionManagerLayer::new(session_store).with_secure(true);

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
        .merge(game_router())
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
