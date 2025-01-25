use leptos::prelude::*;

#[component]
pub fn PlayerStatisticsMain() -> impl IntoView {
    view! {
        <div>
            <p class="text-center">Player statistics</p>

            <div class="w-full flex flex-wrap">
                <div class="w-1/2">
                    <p class="text-center">
                        <b>"Statistics in game"</b>
                    </p>

                    <div title="Ability points">
                        <b>"AP: "</b>
                        "5 "
                        "("
                        <a href="#">"Use"</a>
                        ")"
                    </div>
                    <div title="Race">
                        <b>"Bloodline: "</b>
                        "Unknown "
                        "("
                        <a href="#">Select</a>
                        ")"
                    </div>
                    <div title="Class">
                        <b>"Craft : "</b>
                        "Unknown "
                        "("
                        <a href="#">Select</a>
                        ")"
                    </div>
                    <div>
                        <b>"Religion : "</b>
                        "Unknown "
                        "("
                        <a href="#">Select</a>
                        ")"
                    </div>
                    <div>
                        <b>"Gender : "</b>
                        "Unknown "
                        "("
                        <a href="#">Select</a>
                        ")"
                    </div>
                    <div>
                        <b>"Agility : "</b>
                        "0"
                    </div>
                    <div>
                        <b>"Strength : "</b>
                        "0"
                    </div>
                    <div>
                        <b>"Intelligence : "</b>
                        "0"
                    </div>
                    <div>
                        <b>"Wisdom : "</b>
                        "0"
                    </div>
                    <div>
                        <b>"Speed : "</b>
                        "0"
                    </div>
                    <div>
                        <b>"Condition : "</b>
                        "0"
                    </div>
                    <div title="Mana points">
                        <b>"Spell points: "</b>
                        "0"
                    </div>
                    <div>
                        <b>"Religion points: "</b>
                        "0"
                    </div>
                    <div>
                        <b>"Energy: "</b>
                        "10.00/70.00"
                    </div>
                    <div class="mb-8"></div>

                    <div>"Reputation:"</div>
                    <div>
                        <b>"Fights: "</b>
                        <p class="inline-block" title="Won">
                            "0"
                        </p>
                        <p class="inline-block">"/"</p>
                        <p class="inline-block" title="Lost">
                            "0"
                        </p>
                        <p class="inline-block">"/"</p>
                        <p class="inline-block" title="All">
                            0
                        </p>
                    </div>
                    <div>
                        <b>"Last killed: "</b>
                        "..."
                    </div>
                    <div>
                        <b>"Last killed by: "</b>
                        "..."
                    </div>
                    <div>
                        <b>"Finished quests: "</b>
                        "0"
                    </div>
                </div>
                <div class="w-1/2">
                    <div class="text-center">
                        <b>"Player information"</b>
                    </div>

                    <div>
                        <b>"Rank: "</b>
                        "Admin"
                    </div>
                    <div>
                        <b>"Location: "</b>
                        "Altara"
                    </div>
                    <div>
                        <b>"Age: "</b>
                        "1"
                    </div>
                    <div>
                        <b>"Visits: "</b>
                        "1"
                    </div>
                    <div>
                        <b>"IP: "</b>
                        "10.0.0.1"
                    </div>
                    <div>
                        <b>"E-mail: "</b>
                        "admin@vallheru.pl"
                    </div>
                    <div>
                        <b>"Young player protection: "</b>
                        "3 days"
                        "("
                        <a href="#">"Disable"</a>
                        ")"
                    </div>
                    <div>
                        <b>"Tribe: "</b>
                        "none"
                    </div>
                </div>
                <div class="mt-16 w-full flex flex-wrap">
                    <div class="w-1/2">
                        <div class="text-center">
                            <b>"Talents"</b>
                        </div>
                    </div>
                    <div class="w-1/2">
                        <div class="text-center">
                            <b>"Bonuses"</b>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// define("T_CLASS2", "Klasa");
// define("T_DEITY", "Wyznanie");
// define("T_GENDER", "Płeć");
// define("T_AGI", "Zręczność");
// define("T_STR", "Siła");
// define("T_INT", "Inteligencja");
// define("T_WIS", "Siła Woli");
// define("T_SPEED", "Szybkość");
// define("T_CON", "Wytrzymałość");
// define("T_MANA", "Punkty Magii");
// define("T_PW", "Punkty Wiary");
// define("T_FIGHTS", "Wyniki");
// define("T_LAST", "Ostatnio zabity");
// define("T_LAST2", "Ostatnio zabity przez");
