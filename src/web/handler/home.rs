use axum::Form;
use tower_sessions::Session;
use crate::{controller::login_controller, player_state::PlayerState, web::{form::LoginForm, handler::prelude::*}};

pub async fn get_news(
    State(state): State<AppState>,
    player_state: PlayerState,
) -> Html<String> {
    let template = state.tpl_env.get_template("news.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {}))
        .unwrap();
    Html(r)
}

pub async fn get_index(
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

pub async fn post_login(
    State(state): State<AppState>,
    session: Session,
    Form(login): Form<LoginForm>,    
) -> Html<String> {
    let res = login_controller(session, &state.db, &login.email, &login.password).await;

    match res {
        Err(e) => state.render("home_login.html", context!{ login_error => e }),
        Ok(player) => state.render("home_login_welcome.html", context!{
            player_id => player.id, 
            player_name => player.username,
            login_count => player.login_count,
        })
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