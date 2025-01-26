use leptos::prelude::*;

#[component]
pub fn Row(
    name: &'static str,
    #[prop(default = "")] title: &'static str,
    value: impl IntoView,

    #[prop(default = "")] additional_action_text: &'static str,
    #[prop(default = "#")] additional_action_path: &'static str,
    #[prop(default = None)] action_activation_signal: Option<WriteSignal<bool>>,
) -> impl IntoView {
    let activate_additional_action = move |_| {
        if action_activation_signal.is_none() {
            return;
        }
        action_activation_signal.unwrap().set(true);
    };

    view! {
        <p title=if !title.is_empty() { title } else { "" }>
            <b>{name}": "</b>
            {value}

            <Show when=move || {
                !additional_action_text.is_empty()
            }>
                " (" <a on:click=activate_additional_action href=additional_action_path>
                    {additional_action_text}
                </a> ")"
            </Show>
        </p>
    }
}

#[component]
pub fn PlayerStatisticsMain() -> impl IntoView {
    let (use_ap, set_use_ap) = signal(false);
    let (select_bloodline, set_select_bloodline) = signal(false);
    let (select_class, set_select_class) = signal(false);
    let (select_religion, set_select_religion) = signal(false);
    let (select_gender, set_select_gender) = signal(false);
    let (disable_protection, set_disable_protection) = signal(false);

    view! {
        <div>
            <p class="text-center">Player statistics</p>

            <div class="w-full flex flex-wrap">
                <div class="w-1/2">
                    <p class="text-center">
                        <b>"Statistics in game"</b>
                    </p>

                    <Row
                        title="Ability points"
                        name="AP"
                        value=view! { "5" }
                        additional_action_text="Use"
                        additional_action_path="/player-statistics/use-ap"
                        action_activation_signal=Some(set_use_ap)
                    />
                    <Row
                        title="Race"
                        name="Bloodline"
                        value=view! { "..." }
                        additional_action_text="Select"
                        additional_action_path="/player-statistics/select-bloodline"
                        action_activation_signal=Some(set_select_bloodline)
                    />
                    <Row
                        title="Class"
                        name="Craft"
                        value=view! { "..." }
                        additional_action_text="Select"
                        additional_action_path="/player-statistics/select-class"
                        action_activation_signal=Some(set_select_class)
                    />
                    <Row
                        name="Religion"
                        value=view! { "..." }
                        additional_action_text="Select"
                        additional_action_path="/player-statistics/select-religion"
                        action_activation_signal=Some(set_select_religion)
                    />
                    <Row
                        name="Gender"
                        value=view! { "..." }
                        additional_action_text="Select"
                        additional_action_path="/player-statistics/select-gender"
                        action_activation_signal=Some(set_select_gender)
                    />
                    <Row name="Agility" value=view! { "0" } />
                    <Row name="Strength" value=view! { "0" } />
                    <Row name="Intelligence" value=view! { "0" } />
                    <Row name="Wisdom" value=view! { "0" } />
                    <Row name="Speed" value=view! { "0" } />
                    <Row name="Condition" value=view! { "0" } />
                    <Row title="Mana points" name="Spell points" value=view! { "0" } />
                    <Row name="Religion points" value=view! { "0" } />
                    <Row name="Energy" value=view! { "212.00/1500" } />
                    <div class="mb-8"></div>

                    <div class="mb-2">
                        <b>"Reputation:"</b>
                    </div>
                    <Row
                        name="Fights"
                        value=view! {
                            <p class="inline" title="Won">
                                "0"
                            </p>
                            <p class="inline">"/"</p>
                            <p class="inline" title="Lost">
                                "0"
                            </p>
                            <p class="inline">"/"</p>
                            <p class="inline" title="All">
                                0
                            </p>
                        }
                    />

                    <Row name="Last killed" value=view! { "..." } />
                    <Row name="Last killed by" value=view! { "..." } />
                    <Row name="Finished quests" value=view! { "..." } />
                </div>

                <div class="w-1/2">
                    <div class="text-center">
                        <b>"Player information"</b>
                    </div>

                    <Row name="Rank" value=view! { "Admin" } />
                    <Row name="Location" value=view! { "Altara" } />
                    <Row name="Age" value=view! { "1" } />
                    <Row name="Visits" value=view! { "1" } />
                    <Row name="IP" value=view! { "10.0.0.1" } />
                    <Row name="E-mail" value=view! { "admin@vallheru.pl" } />

                    <Row
                        name="Young player protection"
                        value=view! { "3 days" }
                        additional_action_text="Disable"
                        additional_action_path="/player-statistics/disable-protection"
                        action_activation_signal=Some(set_disable_protection)
                    />

                    <Row name="Tribe" value=view! { "..." } />
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
