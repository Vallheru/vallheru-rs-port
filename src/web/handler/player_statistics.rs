use crate::{web::handler::prelude::*, player_state::PlayerState, web::AppState};

pub async fn get_player_statistics(
    State(state): State<AppState>,
    player_state: PlayerState,
) -> Html<String> {
    let template = state.tpl_env.get_template("player_statistics.html").unwrap();
    let r = template
        .render(player_state.game_context())
        .unwrap();
    
    Html(r)
}