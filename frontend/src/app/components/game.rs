use leptos::prelude::*;
use leptos_meta::*;
use vallheru::api::{player::PlayerIdentifier, PlayerRequest, PlayerResponse};

use crate::{api::Client, player_state::Context};

#[component]
pub fn GameMain() -> impl IntoView {
    let is_token_valid_resource = LocalResource::new(async move || {
        let client = Client::new().with_stored_token::<Context>();

        client.validate_token().await
    });

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Body attr:class="bg-black text-vallheru" />

        <div class="flex game-window content-center justify-center">
            <div class="max-w-screen-xl h-full flex flex-wrap">
                <Suspense fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || Suspend::new(async move {
                        let result = is_token_valid_resource.await;

                        view! {
                            <Show
                                when=move || { result.is_valid }
                                fallback=|| view! { <InvalidToken /> }
                            >
                                <GameTemplate />
                            </Show>
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
fn InvalidToken() -> impl IntoView {
    let set_player_context = use_context::<WriteSignal<Context>>()
        // we know we just provided this in the parent component
        .expect("there to be a `count` signal provided");

    let logout = move |_| {
        set_player_context.set(Context::new());
    };

    view! {
        <p>
            "Your session expired. Click "<a href="/logout" on:click=logout>
                "here"
            </a>" to log in"
        </p>
    }
}

#[component]
fn GameTemplate() -> impl IntoView {
    // Also client is
    let client = Client::new().with_stored_token::<Context>(); // may panic. Token must be obtained from the player context
    provide_context(client.clone());

    let player_resource = LocalResource::new(async move || {
        let client = Client::new().with_stored_token::<Context>();
        client
            .send_and_unwrap::<_, PlayerResponse>(
                &PlayerRequest {
                    identifier: PlayerIdentifier::AuthToken,
                },
                |err| leptos::logging::log!("failed to get player resource: {:?}", err),
            )
            .await
    });

    view! {
        <div class="w-full text-center border-1 border-game-border p-2">
            <GameHeader />
        </div>

        <div class="w-52 border-1 border-game-border flex-grow-0">
            <PlayerStatistics resource=player_resource />

            <GameLinks />
        </div>

        <div class="flex-grow flex-shrink basis-80 p-4">
            <GameContent />
        </div>

        <div class="w-52 border-1 border-game-border flex-grow-0">
            <ListOnlinePlayers />
        </div>
    }
}

#[component]
fn GameHeader() -> impl IntoView {
    view! {
        <div>
            <b>"Time in game:"</b>
            "19:20:03"
            <b>" Full moon"</b>
        </div>
    }
}

#[component]
fn PlayerStatistics(resource: LocalResource<Option<PlayerResponse>>) -> impl IntoView {
    view! {
        <div>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <p class="text-center">
                    <b>"Player stats"</b>
                </p>
                {move || Suspend::new(async move {
                    let player = resource.await.unwrap();
                    leptos::logging::log!("{:?}", player);

                    view! {
                        <p class="text-center">
                            <b>{player.player.username}</b>
                            " ("
                            {player.player.id}
                            ")"
                        </p>
                        <br />

                        <p>
                            <b>"HP: "</b>
                            {player.player.hp}
                            "/"
                            {player.player.max_hp}
                        </p>
                        <p>
                            <b>"SP: "</b>
                            {player.player.sp}
                        </p>
                        <p>
                            <b>"Energy: "</b>
                            "212.00/1500"
                        </p>
                        <br />

                        <p>
                            <b>"Gold: "</b>
                            {player.player.gold}
                        </p>
                        <p>
                            <b>"Bank: "</b>
                            {player.player.bank}
                        </p>
                        <p>
                            <b>"Mithrill: "</b>
                            {player.player.mithrill}
                        </p>
                        <p>
                            <b>"Vallars: "</b>
                            {player.player.vallars}
                        </p>
                    }
                })}

            </Suspense>
        </div>
    }
}

#[component]
fn GameLinks() -> impl IntoView {
    view! {
        <div class="ml-4 mt-8">
            <p>
                <b>"Links"</b>
            </p>
            <ul>
                <li>
                    <a href="#">"Player"statistics</a>
                </li>
                <li>
                    <a href="#">"Minerals"</a>
                </li>
                <li>
                    <a href="#">"Equipment"</a>
                </li>
                <li class="mb-2">
                    <a href="#">"Notes"</a>
                </li>

                <li>
                    <a href="#">"Altara"</a>
                </li>
                <li>
                    <a href="#">"Fighting arena"</a>
                </li>
                <li>
                    <a href="#">"Hospital"</a>
                </li>
                <li class="mb-2">
                    <a href="#">"Bank"</a>
                </li>

                <li>
                    <a href="#">"Post office"</a>
                    " [0]"
                </li>
                <li>
                    <a href="#">"Forum"</a>
                    " [0]"
                </li>
                <li>
                    <a href="#">"Inn"</a>
                    " [0]"
                </li>

            </ul>
        </div>
    }
}

#[component]
fn GameContent() -> impl IntoView {
    view! {
        <div>
            <p class="text-center">Library</p>
            <p>
                "Welcome to the Library, a sanctuary of stories and verses left behind by travelers who have
                wandered far and wide along life's many paths. Here, amidst the quiet hum of ancient tomes and 
                the gentle flicker of candlelight, you’ll discover tales of heroism, whispers of lost legends, 
                and poems that capture the very essence of the soul."
            </p>
            <p>
                "If you so desire, you may add your own voice to this collection, weaving your story into
                the fabric of this hallowed place. However, be mindful, for the Library is under the watchful 
                care of the Librarian, a mysterious keeper of wisdom and guardian of these treasures. It is the
                Librarian who determines which works are worthy to grace these shelves, preserving the Library’s 
                integrity for generations to come."
            </p>
            <p>
                "All rights to the texts remain with their creators, ensuring their legacy endures. At present,
                the Library holds [number] texts, while [number] more await the Librarian’s judgment to join this 
                esteemed archive"
            </p>
        </div>
    }
}

#[component]
fn ListOnlinePlayers() -> impl IntoView {
    view! {
        <div>
            <p class="text-center">"Game statistics"</p>
            <p class="mb-6">"List of players in the game:"</p>

            <ul>
                <li>
                    <a href="#">"admin"</a>
                    " (1)"
                </li>
                <li>
                    <a href="#">"Aeris Shadowbane"</a>
                    " (2)"
                </li>
                <li>
                    <a href="#">"Kael Stormrider"</a>
                    " (3)"
                </li>
                <li>
                    <a href="#">"Lyria Moonweaver"</a>
                    " (4)"
                </li>
                <li>
                    <a href="#">"Draven Nightthorn"</a>
                    " (5)"
                </li>
                <li>
                    <a href="#">"Sylara Frostsong"</a>
                    " (6)"
                </li>
                <li>
                    <a href="#">"Thalric Ironflame"</a>
                    " (7)"
                </li>
                <li>
                    <a href="#">"Zyra Emberleaf"</a>
                    " (8)"
                </li>
                <li>
                    <a href="#">"Corwyn Duskbringer"</a>
                    " (9)"
                </li>
                <li>
                    <a href="#">"Eryndor Starforge"</a>
                    " (10)"
                </li>
                <li>
                    <a href="#">"Veyra Silverveil"</a>
                    " (11)"
                </li>
            </ul>
        </div>
    }
}
