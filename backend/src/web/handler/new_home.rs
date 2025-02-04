use std::sync::Arc;

use axum::Form;
use minijinja::context;
use serde::{Deserialize, Serialize};
use vallheru::model::Player;

use crate::{repository::{player::{alter_last_login_and_login_count, get_player_by_email}, token::{create_token_for_player, get_active_token_for_player}}, web::{form::LoginForm, handler::prelude::*}};

pub async fn get_player(
    Extension(this_player): Extension<Arc<Player>>,
    Path(identifier): Path<String>,
) -> String {
    "test".into()
}

pub async fn index(
    State(state): State<AppState>
) -> Html<String> {
    let template = state.tpl_env.get_template("home_index.html").unwrap();
    let r = template
        .render(context! {})
        .unwrap();
    Html(r)
}

pub async fn get_login(
    State(state): State<AppState>
) -> Html<String> {
    let template = state.tpl_env.get_template("home_login.html").unwrap();
    let r = template
        .render(context! {})
        .unwrap();
    Html(r)
}

#[derive(Serialize, Deserialize)]
enum LoginErrorKind {
    InvalidCredentials,
    SessionSetupError,
}

pub async fn post_login(
    State(state): State<AppState>,
    Form(login): Form<LoginForm>
) -> Html<String> {
    let player = get_player_by_email(&state.db, &login.email, &login.password).await;

    if player.is_err() {
        return state.render("home_login.html", context!{ login_error => LoginErrorKind::InvalidCredentials })
    }

    let player = player.unwrap();
    let mut token = get_active_token_for_player(&state.db, player.id).await;

    if token.is_none() {
        token = create_token_for_player(&state.db, player.id).await;
    }

    match token {
        Some(token) => {
            let _ = alter_last_login_and_login_count(&state.db, player.id).await; // TODO: check that error

            state.render("home_login_welcome.html", context!{
                player_id => player.id, 
                player_name => player.username,
                login_count => player.login_count,
            })
        }
        None => state.render("home_login.html", context!{ login_error => LoginErrorKind::SessionSetupError }),
    }
}


pub async fn get_register(
    State(state): State<AppState>
) -> Html<String> {
    let template = state.tpl_env.get_template("home_register.html").unwrap();
    let r = template
        .render(context! {})
        .unwrap();
    Html(r)
}

// pub async fn login()