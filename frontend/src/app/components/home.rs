use leptos::{html, logging::log, prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use vallheru::{api::LoginResponse, name_generator::random_name};
use web_sys::SubmitEvent;

use crate::player_state;

const COMMON_FORM_CLASS: &str = "px-4 py-2 rounded-md border-2 border-[#1e1a20] bg-gradient-to-r from-vallheru-creme-100 to-vallheru-creme-400";
const COMMON_BUTTON_CLASS: &str =
    "bg-[#322c33] text-xl text-white rounded-full py-3 px-10 hover:text-black";

#[component(transparent)]
fn HomeRouterContent() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/register") view=Register/>
                <Route path=path!("/login") view=Login/>
                <Route path=path!("/rules") view=Rules/>
                <Route path=path!("/faq") view=Faq/>
                <Route path=path!("/links") view=Links/>
                <Route path=path!("/news") view=ReadNews/>
                <Route path=path!("/reset-password") view=ResetPassword/>
                <Route path=path!("/welcome") view=Welcome/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn HomeTemplate() -> impl IntoView {
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Body
            attr:class="text-zinc-400 min-h-screen w-full
                bg-[#d0cabd] bg-image-home bg-left-bottom bg-no-repeat bg-cover 2xl:bg-cover md:bg-contain sm:bg-contain portrait:bg-contain" />

            <div class="w-full h-full relative pt-12 pb-56">
                <div>
                    <Navigation />
                </div>

                <div class="alegreya-sc-medium-italic mt-12 mb-20 text-center text-black">
                    <p class="text-6xl font-bold py-4">Welcome to Vallheru</p>
                    <p class="text-2xl">A Realm of Heroes, Mysteries, and Endless Adventure!</p>
                </div>

                <div>
                    <Content />
                </div>
            </div>
    }
}

#[component]
fn NavigationLink(
    link: String,
    activate_links: Vec<String>,
    name: String,
    cur_page: ReadSignal<String>,
    set_cur_page: WriteSignal<String>,
) -> impl IntoView {
    let link_copy = link.clone();

    view! {
        <li class=move || {if &cur_page.get() == &link_copy {"font-bold"} else {""}}>
            <a href={link.clone()} on:click=move |_| { set_cur_page.set(link.to_string()) }>{name}</a>
        </li>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    let (cur_page, set_cur_page) = signal(location().pathname().unwrap_or("".into()));

    view! {
        <div class="w-full flex items-center">
            <p class="alegreya-sc-medium-italic text-left text-black text-5xl mr-6 ml-14">
                "Vallheru"
            </p>


            <ul class="w-full alegreya-sc-medium-italic flex justify-center gap-x-6 -ml-56 text-black text-3xl">
                <NavigationLink
                    link=String::from("/")
                    activate_links=vec![String::from("/"), String::from("/news")]
                    name=String::from("Home")
                    cur_page=cur_page
                    set_cur_page=set_cur_page />

                <NavigationLink
                    link=String::from("/faq")
                    activate_links=vec![String::from("/faq")]
                    name=String::from("FAQ")
                    cur_page=cur_page
                    set_cur_page=set_cur_page />

                <NavigationLink
                    link=String::from("/rules")
                    activate_links=vec![String::from("/rules")]
                    name=String::from("Rules")
                    cur_page=cur_page
                    set_cur_page=set_cur_page />

                <NavigationLink
                    link=String::from("/login")
                    activate_links=vec![String::from("/login")]
                    name=String::from("Login")
                    cur_page=cur_page
                    set_cur_page=set_cur_page />

                    <NavigationLink
                    link=String::from("/register")
                    activate_links=vec![String::from("/register")]
                    name=String::from("Register")
                    cur_page=cur_page
                    set_cur_page=set_cur_page />

                <NavigationLink
                    link=String::from("/links")
                    activate_links=vec![String::from("/links")]
                    name=String::from("Links")
                    cur_page=cur_page
                    set_cur_page=set_cur_page />
            </ul>
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="w-full flex justify-center px-8">
            <HomeRouterContent />
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="lg:w-3/5 md:w-full text-black alegreya-sc-medium-italic">
            <div class="text-4xl text-center mb-4">"The Realm's Whisper"</div>

            <div class="mb-8">
                <h2 class="text-xl pb-4">Exploring the Depths</h2>
                <p class="text-justify">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer vehicula, augue
                    eu interdum cursus, turpis ligula fermentum eros, a sagittis mauris est non enim.
                    Ut et tortor at nisi gravida venenatis. Aliquam erat volutpat. Aenean hendrerit
                    malesuada enim vel sagittis. Donec id ligula in nunc blandit cursus. Curabitur
                    accumsan lectus ut sapien vestibulum, et tempus metus laoreet. Nam posuere, sapien
                    a iaculis varius, lacus purus efficitur dolor, id tempus enim magna a ligula.
                </p>
                <p class="text-right mt-2"><a href="/news">Read more</a></p>
            </div>

            <div class="mb-8">
                <h2 class="text-xl pb-4">The Rise of Heroes</h2>
                <p class="text-justify">
                    orbi ut eros dignissim, tincidunt sem id, volutpat sapien. Aenean condimentum,
                    metus vitae consectetur pretium, purus nisl scelerisque nulla, nec viverra arcu
                    nunc a velit. Sed faucibus, augue vel faucibus ultricies, est libero vehicula metus,
                    in aliquam dolor nulla vel nunc. In dictum, neque non tempus gravida, nulla ipsum
                    interdum ligula, ut eleifend erat justo nec risus.
                </p>
                <p class="text-right mt-2"><a href="/news">Read more</a></p>
            </div>

            <div class="mb-8">
                <h2 class="text-xl pb-4">Unveiling the Secrets</h2>
                <p class="text-justify">
                    "Aliquam malesuada varius felis, at pretium mi dignissim quis. Nunc facilisis sapien
                    eu augue fermentum, nec fringilla tortor vehicula. Integer ullamcorper, nunc quis
                    gravida convallis, nisi erat fermentum urna, eu pellentesque purus felis non ex.
                    Nam tincidunt, magna a ultrices pharetra, orci metus pellentesque justo, in mollis
                    nulla justo nec nisi. Donec at venenatis augue. Vivamus ultricies odio eget tortor
                    aliquam tristique. Nulla facilisi. Phasellus fermentum purus in neque fermentum,
                    ut hendrerit dolor rhoncus."
                </p>
                <p class="text-right mt-2"><a href="/news">Read more</a></p>
            </div>
        </div>
    }
}

#[component]
fn Register() -> impl IntoView {
    let (name, set_name) = signal("".to_string());

    view! {
        <div class="lg:w-3/5 md:w-full text-black alegreya-sc-medium-italic">
            <div class="text-4xl text-center mb-4">"Enter Vallheru"</div>

            <p class="text-lg">
                "Welcome to the gateway of endless adventure and fantasy!
                Create your account now to step into the immersive world of 
                Vallheru, where heroes are forged, alliances are built, and 
                legends are born. Simply provide your email address, select a 
                unique fantasy nickname (or let our generator spark your imagination), 
                and set a strong, secure password to protect your account. Confirm your 
                password for added security, then click "Register" to embark on your quest."
                <br /><br />
                "After completing this step, an activation email will be sent to the address you
                provided. Be sure to check your inbox (and spam folder, just in case) and follow 
                the instructions to activate your account. Once activated, the realm of Vallheru 
                will be yours to explore. Your adventure awaits!"
            </p>

            <form class="max-w-sm mx-auto mt-14">
                <div class={"flex flex-row mb-6 ".to_owned() + COMMON_FORM_CLASS}>
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

                <div class={"flex flex-row mb-6 text-black ".to_owned() + COMMON_FORM_CLASS}>
                    <label for="pass" class="text-black pr-3">Password: </label>
                    <input
                        type="password"
                        id="pass"
                        class="text-black bg-transparent outline-none border-none"
                        required />
                </div>

                <div class={"flex flex-row mb-6 text-black ".to_owned() + COMMON_FORM_CLASS}>
                    <label for="re-pass" class="text-black pr-3">Re password: </label>
                    <input
                        type="password"
                        id="re-pass"
                        class="w-28 text-black bg-transparent outline-none border-none"
                        required />
                </div>

                <div class="mb-12">
                    <input type="checkbox" id="rules" class="w-6 h-6 border-none bg-vallheru-creme-100 text-black accent-vallheru-creme-300" />
                    <label for="rules" class="text-2xl pl-2">I accept the rules</label>
                </div>

                <div class="mb-3">
                    <button class={"".to_owned() + COMMON_BUTTON_CLASS }>
                        Register
                    </button>
                </div>

            </form>
        </div>
    }
}

#[component]
fn Login() -> impl IntoView {
    let email_element: NodeRef<html::Input> = NodeRef::new();
    let password_element: NodeRef<html::Input> = NodeRef::new();
    let (disabled_button, set_disabled_button) = signal(false);

    let login_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let email = email_element
            .get()
            .expect("<input email> should be mounted")
            .value();
        let pass = password_element
            .get()
            .expect("<input email> should be mounted")
            .value();

        spawn_local(async move {
            println!("Sending");
            set_disabled_button.set(true);
            // let req = crate::app::api::login(&email, &pass).await.unwrap();

            let req: LoginResponse = vallheru::api::api_request(
                None,
                "http://nebula-dev.local.mainnet.community:3004",
                &vallheru::api::LoginRequest {
                    email,
                    password: pass,
                },
            )
            .await
            .unwrap();

            set_disabled_button.set(false);
            leptos::logging::log!("{:?}", req);
        });
    };

    view! {
        <div class="lg:w-3/5 md:w-full text-black alegreya-sc-medium-italic">
            <div class="text-4xl text-center mb-4">"Enter Vallheru"</div>

            <p class="text-lg text-center mt-8">Step into a world of imagination and adventure. Log in to continue your journey!</p>

            <form
                on:submit=login_submit
                class="max-w-sm mx-auto mt-14">
                <div class={"mb-8 ".to_owned() + COMMON_FORM_CLASS}>
                    <label for="login-email" class="text-black pr-3">Email: </label>
                    <input
                        type="email"
                        id="login-email"
                        class="text-black bg-transparent outline-none border-none"
                        placeholder="example@email.com"
                        node_ref=email_element
                        required />
                </div>

                <div class={"mb-12 ".to_owned() + COMMON_FORM_CLASS}>
                    <label for="login-password" class="text-black pr-3">Password: </label>
                    <input
                        type="password"
                        id="login-password"
                        class="text-black bg-transparent outline-none border-none"
                        placeholder="password"
                        node_ref=password_element
                        required />
                </div>

                <div class="mb-3">
                    <button
                        class={"".to_owned() + COMMON_BUTTON_CLASS }
                        disabled=move || disabled_button.get()>
                        Log in
                    </button>
                </div>

                <p class="alegreya-sc-medium-italic">
                    <a href="/reset-password">Reset password</a>
                </p>
            </form>
        </div>
    }
}

#[component]
fn Rules() -> impl IntoView {
    view! {
        <p class="text-xl">
            Rules
        </p>
    }
}

#[component]
fn Faq() -> impl IntoView {
    view! {
        <p class="text-xl">
            FAQ
        </p>
    }
}

#[component]
fn Links() -> impl IntoView {
    view! {
        <p class="text-xl">
            Links
        </p>
    }
}

#[component]
fn ReadNews() -> impl IntoView {
    view! {
        <div class="lg:w-3/5 md:w-full text-black alegreya-sc-medium-italic">
            <div class="text-4xl text-center mb-4">"The Realm's Whisper"</div>

            <div class="mb-8">
                <h2 class="text-xl pb-4">Exploring the Depths</h2>
                <p class="text-justify">
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque a vehicula nulla.
                Fusce vitae urna in magna posuere pretium. Sed dapibus, augue sed tristique vulputate, 
                magna lorem elementum lorem, sit amet tristique purus nisi ac lacus. Nullam hendrerit 
                lacus sed mauris efficitur, nec dictum urna consequat. Suspendisse potenti. Nunc 
                aliquet sapien vel ante hendrerit, ac tempor urna placerat. Integer porta libero nec 
                eros ultricies tincidunt."
                <br /><br />
                "Vivamus rutrum lorem eu mi bibendum condimentum. Sed auctor urna ac urna dictum fringilla.
                Duis blandit, magna ac sollicitudin ultricies, enim metus viverra neque, id accumsan eros risus 
                ut orci. Cras rhoncus urna vel volutpat vehicula. Etiam sed massa sit amet lorem euismod eleifend. 
                Mauris semper felis ut lorem feugiat, at posuere neque dapibus. Integer accumsan magna ut eros 
                tristique vehicula."
                <br /><br />
                "Sed nec est quis lacus gravida tincidunt. Etiam vitae orci at libero efficitur hendrerit
                sed sed nisl. Nullam fermentum sapien justo, ac malesuada augue faucibus eget. Integer eu 
                est eros. Ut congue dui eu vehicula feugiat. Morbi tincidunt nunc et quam cursus, nec congue 
                tortor suscipit. Fusce id lorem ac lorem finibus vestibulum. Suspendisse potenti."
                <br /><br />
                "Aliquam malesuada varius felis, at pretium mi dignissim quis. Nunc facilisis sapien eu
                augue fermentum, nec fringilla tortor vehicula. Integer ullamcorper, nunc quis gravida convallis, 
                nisi erat fermentum urna, eu pellentesque purus felis non ex. Nam tincidunt, magna a ultrices 
                pharetra, orci metus pellentesque justo, in mollis nulla justo nec nisi. Donec at venenatis augue. 
                Vivamus ultricies odio eget tortor aliquam tristique. Nulla facilisi. Phasellus fermentum purus in 
                neque fermentum, ut hendrerit dolor rhoncus."
                </p>
            </div>
        </div>
    }
}

#[component]
fn ResetPassword() -> impl IntoView {
    view! {
        <div>ResetPassword</div>
    }
}

#[component]
fn Welcome() -> impl IntoView {
    view! {
        <div>Welcome</div>
    }
}
