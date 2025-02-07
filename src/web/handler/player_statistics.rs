use crate::{player_state::PlayerState, repository::player::disable_player_protection, web::{handler::prelude::*, AppState}};

pub async fn get_player_statistics(
    State(state): State<AppState>,
    player_state: PlayerState,
) -> Html<String> {
    let template = state.tpl_env.get_template("player_statistics/main.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {}))
        .unwrap();
    
    Html(r)
}

pub async fn get_use_ap(
    State(state): State<AppState>,
    player_state: PlayerState,
) -> Html<String> {
    let template = state.tpl_env.get_template("player_statistics/use_ap.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {}))
        .unwrap();
    
    Html(r)
}

pub async fn get_select_bloodline(
    State(state): State<AppState>,
    player_state: PlayerState,
) -> Html<String> {
    let template = state.tpl_env.get_template("player_statistics/select_bloodline.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {}))
        .unwrap();
    
    Html(r)
}

pub async fn get_select_class(
    State(state): State<AppState>,
    player_state: PlayerState,
) -> Html<String> {
    let template = state.tpl_env.get_template("player_statistics/select_class.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {}))
        .unwrap();
    
    Html(r)
}

#[derive(Deserialize, Debug)]
pub struct SelectReligion {
    select: Option<String>,
    confirm: Option<i32>,
}

pub async fn get_select_religion(
    State(state): State<AppState>,
    player_state: PlayerState,
    params: Query<SelectReligion>,
) -> Html<String> {
    let selected_religion = params.select.clone().unwrap_or_default();

    let template = state.tpl_env.get_template("player_statistics/select_religion.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {
            select => &selected_religion,
        }))
        .unwrap();
    
    Html(r)
}

pub async fn get_select_gender(
    State(state): State<AppState>,
    player_state: PlayerState,
) -> Html<String> {
    let template = state.tpl_env.get_template("player_statistics/select_gender.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {}))
        .unwrap();
    
    Html(r)
}

#[derive(Deserialize, Debug)]
pub struct DisableProtectionQuery {
    confirm: Option<i32>
}

pub async fn get_disable_protection(
    State(state): State<AppState>,
    player_state: PlayerState,
    params: Query<DisableProtectionQuery>,
) -> Html<String> {
    let player_id = &player_state.player.clone().unwrap().id;

    if params.confirm.is_some() {
        if let Err(e) = disable_player_protection(&state.db, player_id).await {
            println!("Disable protection - {:?}", e);
            return player_state.render_error(&state.tpl_env, "Internal server error(1)");
        }
    }

    let template = state.tpl_env.get_template("player_statistics/disable_protection.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {
            confirmed => params.confirm.is_some()
        }))
        .unwrap();
    
    Html(r)
}