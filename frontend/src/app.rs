pub mod components;
pub mod player_state;

use components::HomeTemplate;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(player_state::Context::default()));
    let (player_context, set_player_context) = signal(player_state::Context::default());

    provide_context(set_player_context);

    Effect::new(move |_| {
        if let Ok(Some(stor)) = window().local_storage() {
            let json =
                serde_json::to_string(&player_context).expect("could not serialize PlayerContext");

            if stor.set_item(player_state::STORAGE_KEY, &json).is_err() {
                leptos::logging::error!("failed to save player context to local storage");
            }
        }
    });

    let log_out = move |_| set_player_context.update(|ps| ps.auth = "".to_string());

    view! {
        <Show
            when=move || { !player_context.get().auth.is_empty() }
            fallback=|| view! { <HomeTemplate /> }
        >
            <div>Logged in, <a on:click=log_out>Log out</a></div>
        </Show>
    }
}
