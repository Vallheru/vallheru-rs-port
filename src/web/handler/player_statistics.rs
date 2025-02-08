use crate::{model::player::PlayerReligion, player_state::PlayerState, repository::player::{disable_player_protection, set_religion}, web::{handler::prelude::*, AppState}};
use std::str::FromStr;

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
pub struct SelectReligionQuery {
    select: Option<String>,
    confirm: Option<i32>,
}

pub async fn get_select_religion(
    State(state): State<AppState>,
    player_state: PlayerState,
    params: Query<SelectReligionQuery>,
) -> Html<String> {
    let this_player = player_state.player.clone().unwrap();
    let selected_religion = params.select.clone().unwrap_or_default();
    let player_id = this_player.id;
    let player_religion = this_player.religion;

    if player_religion != PlayerReligion::Atheist {
        return player_state.render_error(&state.tpl_env, "Deity already selected", "/game/player-statistics");
    }

    if params.confirm.is_some() && params.select.is_some() {
        let deity = PlayerReligion::from_str(&selected_religion);
        match deity {
            Err(_) => {
                return player_state.render_error(&state.tpl_env, "Invalid deity selected", "/game/player-statistics/select-religion");
            },
            Ok(deity) => {
                if let Err(e) = set_religion(&state.db, player_id, deity).await {
                    println!("Select religion - {:?}", e);
                    return player_state.render_error(&state.tpl_env, "Database error(2)", "/game/player-statistics/select-religion");
                }
            }
        }
    }

    let template = state.tpl_env.get_template("player_statistics/select_religion.html").unwrap();
    let r = template
        .render(player_state.game_context(context! {
            select => &selected_religion,
            confirmation => params.confirm.is_some(),
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
    let player_id = player_state.player.clone().unwrap().id;

    if params.confirm.is_some() {
        if let Err(e) = disable_player_protection(&state.db, player_id).await {
            println!("Disable protection - {:?}", e);
            return player_state.render_error(&state.tpl_env, "Database error(1)", "/game/player-statistics/disable-protection");
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