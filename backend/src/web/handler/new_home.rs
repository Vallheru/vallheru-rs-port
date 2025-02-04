use std::sync::Arc;

use vallheru::model::Player;

use crate::web::handler::prelude::*;

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
        .render(minijinja::context! { some_var => String::from("Test") })
        .unwrap();
    Html(r)
}

pub async fn get_login(
    State(state): State<AppState>
) -> Html<String> {
    let template = state.tpl_env.get_template("home_login.html").unwrap();
    let r = template
        .render(minijinja::context! { some_var => String::from("Test") })
        .unwrap();
    Html(r)
}


pub async fn get_register(
    State(state): State<AppState>
) -> Html<String> {
    let template = state.tpl_env.get_template("home_register.html").unwrap();
    let r = template
        .render(minijinja::context! { some_var => String::from("Test") })
        .unwrap();
    Html(r)
}

// pub async fn login()