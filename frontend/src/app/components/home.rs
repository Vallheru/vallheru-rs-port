use crate::app::player_state::ContextStoreFields; // generated
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use reactive_stores::Store;
use vallheru::name_generator::random_name;

use crate::player_state;

const COMMON_FORM_CLASS: &str = "alegreya-sc-medium-italic border-vallheru-creme-300 
focus:ring-vallheru-creme-100 focus:border-vallheru-creme-200 block text-sm rounded-full
outline-none border-2 p-2 px-4 placeholder:text-gray-600 w-full";

#[component]
pub fn HomeTemplate() -> impl IntoView {
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Body
            attr:class="text-zinc-400 min-h-screen w-full
                bg-gradient-to-b from-home-bg-100 from-20% via-home-bg-300 via-50% via-home-bg-500 via-70% via-home-bg-700 to-home-bg-900" />

        <div class="flex min-h-screen w-full items-center justify-center">
            <div class="relative my-12 text-lg text-vallheru-creme-100 h-[779px] lg:w-[1400px] md:w-[800px] sm:w-96">

                <Navigation />

                <div class="h-full w-full rounded-xl bg-home-dragon bg-no-repeat bg-cover bg-center flex flex-row shadow-3xl">
                    <div class="ml-64 w-80 h-full shadow-3xl no-top-bottom-shadow px-1
                    bg-gradient-to-b from-home-content-bg-200 from-70% via-home-content-bg-400 to-95% to-home-content-bg-700">
                        <div class="h-full bg-gradient-to-b from-[#5d4b37] to-[#ddca8f] px-0.5">
                            <div class="relative h-full bg-gradient-to-b from-home-content-bg-200 from-70% via-home-content-bg-400 to-95% to-home-content-bg-700">
                                <div class="pt-20 px-8">
                                    <Content />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component(transparent)]
fn HomeRouterContent() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/register") view=Register/>
                <Route path=path!("/login") view=Login/>
                <Route path=path!("/rules") view=Home/>
                <Route path=path!("/news") view=Home/>
                <Route path=path!("/links") view=Home/>
            </Routes>
        </Router>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    let player_state = use_context::<WriteSignal<player_state::Context>>().unwrap();

    let update = move |_| player_state.update(|ps| ps.auth = "daniel".to_string());

    view! {
        <div class="absolute z-10 h-12 pl-6 pt-3">
            <ul class="alegreya-sc-medium-italic flex h-full w-1/2 items-center gap-x-5">
                <li><a href="/" class="text-[#000011] bg-vallheru-creme-100 hover:bg-vallheru-creme-300 font-bold py-0.5 px-4 rounded-full items-center">Home</a></li>
                <li><a href="/news" class="hover:text-vallheru-creme-300">News</a></li>
                <li><a href="/rules" on:click=update class="hover:text-vallheru-creme-300">Rules</a></li>
                <li><a href="/login" class="ml-12 hover:text-vallheru-creme-300">Login</a></li>
                <li><a href="register" class="hover:text-vallheru-creme-300">Register</a></li>
            </ul>
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="alegreya-sc-medium-italic">
            <HomeRouterContent />
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <p class="text-6xl -ml-2 alegreya-sc-medium-italic">Vallheru</p>
        <p class="text-xl mt-1 alegreya-sc-medium-italic">Land of Dragons</p>
        <p class="text-base text-justify mt-12 alegreya-sc-medium-italic adju">
            "Vallheru is a text-based RPG for multiple players, played in a turn-based system.
            Here, you can fight monsters, duel other players, manage your own stronghold, 
            earn money from your own mine, or create a clan together with other players. 
            Don't expect breathtaking graphics - this game is more about using your imagination. 
            To play, you don't need powerful hardware or any additional programs to download. 
            If what I've written so far has caught your interest, register in the game and 
            join us. However, before you do, make sure to read the code of conduct - the rules 
            that govern the game."
        </p>
    }
}

#[component]
fn Register() -> impl IntoView {
    let (name, set_name) = signal("".to_string());

    view! {
        <p class="text-5xl">Registration</p>
        <p class="text-sm mt-8 text-justify">
        "Create your account to begin your adventure in Vallheru!
        Enter your email, choose a unique fantasy nickname (or
        use our generator for inspiration), and set a secure
        password. Re-enter your password to confirm, then hit
        Register to join the world of imagination and battles!"
        </p>

        <form class="max-w-sm mx-auto mt-14">
            <div class={"flex flex-row bg-vallheru-creme-100 mb-6 ".to_owned() + COMMON_FORM_CLASS}>
                <label for="email" class="text-black pr-3">Email: </label>
                <input
                    type="email"
                    id="email"
                    class="text-black bg-transparent outline-none border-none placeholder:text-gray-600"
                    placeholder="example@email.com"
                    required />
            </div>

            <div class={"flex flex-row bg-vallheru-creme-100 ".to_owned() + COMMON_FORM_CLASS}>
                <label for="nickname" class="text-black pr-3">Nick: </label>
                <input
                    type="text"
                    id="nickname"
                    class="text-black bg-transparent outline-none border-none placeholder:text-gray-600"
                    placeholder="Unique nick name"
                    bind:value=(name, set_name)
                    required />
            </div>

            <div class="flex flex-row text-sm mb-4">
                <a href="#" on:click=move |_| {
                    set_name.set(random_name());
                }>Generate name</a>
            </div>

            <div class={"flex flex-row mb-6 bg-gradient-to-r from-vallheru-creme-100 to-home-content-bg-800 text-black ".to_owned() + COMMON_FORM_CLASS}>
                <label for="pass" class="text-black pr-3">Password: </label>
                <input
                    type="password"
                    id="pass"
                    class="text-black bg-transparent outline-none border-none"
                    required />
            </div>

            <div class={"flex flex-row mb-12 bg-gradient-to-r from-home-content-bg-800 to-vallheru-creme-100 text-black ".to_owned() + COMMON_FORM_CLASS}>
                <label for="re-pass" class="text-black pr-3">Re password: </label>
                <input
                    type="password"
                    id="re-pass"
                    class="w-28 text-black bg-transparent outline-none border-none"
                    required />
            </div>


            <div class="mb-3">
                <button class={"bg-home-content-bg-800 text-vallheru-creme-300 hover:bg-vallheru-creme-100 hover:text-black ".to_owned() + COMMON_FORM_CLASS }>
                    Register
                </button>
            </div>
            <div>
                <input type="checkbox" id="rules" class="border-none bg-vallheru-creme-100 text-black accent-vallheru-creme-300" />
                <label for="rules" class="text-sm pl-2">I accept the rules</label>
            </div>
        </form>
    }
}

#[component]
fn Login() -> impl IntoView {
    view! {
        <p class="text-5xl">Login</p>
        <p class="text-sm mt-8">Step into a world of imagination and adventure. Log in to continue your journey!</p>

        <form class="max-w-sm mx-auto mt-14">
            <div class="mb-8">
                <input type="email" id="email" class={"bg-vallheru-creme-100 text-gray-900 ".to_owned() + COMMON_FORM_CLASS} placeholder="example@email.com" required />
            </div>

            <div class="mb-12">
                <input type="password" id="password" class={"bg-gradient-to-r from-vallheru-creme-100 to-home-content-bg-800 ".to_owned() + COMMON_FORM_CLASS } placeholder="password" required />
            </div>

            <div class="mb-3">
                <button class={"bg-home-content-bg-800 text-vallheru-creme-300 hover:bg-home-content-bg-100".to_owned() + COMMON_FORM_CLASS }>
                    Log in
                </button>
            </div>

            <p class="alegreya-sc-medium-italic text-sm">
                <a href="/reset-password">Reset password</a>
            </p>
        </form>
    }
}

#[component]
fn Rules() -> impl IntoView {
    view! {
        <p class="text-xl">
            Zasady
        </p>
    }
}

#[component]
fn News() -> impl IntoView {
    view! {
        <p class="text-xl">
            Wydarzenia
        </p>
    }
}

#[component]
fn Links() -> impl IntoView {
    view! {
        <p class="text-xl">
            Linki
        </p>
    }
}
