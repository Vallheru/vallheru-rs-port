pub mod api;
pub mod components;
pub mod player_state;
pub mod utils;

use components::GameMain;
use components::HomeTemplate;
use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Default)]
pub struct AppContext {
    global_loading: bool,
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(player_state::Context::default()));
    let (player_context, set_player_context) = signal(player_state::Context::default());
    let (app_context, set_app_context) = signal(AppContext::default());
    provide_context(player_context);
    provide_context(set_player_context);
    provide_context(app_context);
    provide_context(set_app_context);

    Effect::new(move |_| {
        if let Ok(Some(stor)) = window().local_storage() {
            let json =
                serde_json::to_string(&player_context).expect("could not serialize PlayerContext");

            if stor.set_item(player_state::STORAGE_KEY, &json).is_err() {
                leptos::logging::error!("failed to save player context to local storage");
            }
        }
    });

    // let log_out = move |_| set_player_context.set(player_state::Context::new());

    view! {
        <Show when=move || { player_context.get().in_game } fallback=|| view! { <HomeTemplate /> }>
            <GameMain />
        </Show>
    }
}
