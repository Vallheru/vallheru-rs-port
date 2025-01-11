use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn GameTemplate() -> impl IntoView {
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Body
            attr:class="bg-black text-vallheru" />

        <div class="flex game-window content-center justify-center">
            <div class="max-w-screen-xl h-full flex flex-wrap">
                <div class="w-full text-center border-1 border-game-border p-2">
                    <b>"Time in game:"</b> "19:20:03" <b>" Full moon"</b>
                </div>

                <div class="w-52 border-1 border-game-border flex-grow-0">
                    <p class="text-center"><b>"Player stats"</b></p>
                    <p class="text-center"><b>"admin"</b>" (1)"</p>
                    <br />

                    <p><b>"HP:"</b>" 15/15"</p>
                    <p><b>"SP:"</b>" 3"</p>
                    <p><b>"Energy:"</b>" 212.00/1500"</p>
                    <br />

                    <p><b>"Gold:"</b>" 1000"</p>
                    <p><b>"Bank:"</b>" 0"</p>
                    <p><b>"Mithrill:"</b>" 0"</p>
                    <p><b>"Vallars:"</b>" 0"</p>

                    <div class="ml-4 mt-8">
                        <p><b>"Links"</b></p>
                        <ul>
                            <li><a href="#">"Player" statistics</a></li>
                            <li><a href="#">"Minerals"</a></li>
                            <li><a href="#">"Equipment"</a></li>
                            <li class="mb-2"><a href="#">"Notes"</a></li>

                            <li><a href="#">"Altara"</a></li>
                            <li><a href="#">"Fighting arena"</a></li>
                            <li><a href="#">"Hospital"</a></li>
                            <li class="mb-2"><a href="#">"Bank"</a></li>

                            <li><a href="#">"Post office"</a>" [0]"</li>
                            <li><a href="#">"Forum"</a>" [0]"</li>
                            <li><a href="#">"Inn"</a>" [0]"</li>

                        </ul>
                    </div>
                </div>

                <div class="flex-grow flex-shrink basis-80 p-4">
                    <p class="text-center">Library</p>
                    <p>"Welcome to the Library, a sanctuary of stories and verses left behind by travelers who have
                    wandered far and wide along life's many paths. Here, amidst the quiet hum of ancient tomes and 
                    the gentle flicker of candlelight, you’ll discover tales of heroism, whispers of lost legends, 
                    and poems that capture the very essence of the soul."</p>
                    <p>"If you so desire, you may add your own voice to this collection, weaving your story into
                    the fabric of this hallowed place. However, be mindful, for the Library is under the watchful 
                    care of the Librarian, a mysterious keeper of wisdom and guardian of these treasures. It is the
                    Librarian who determines which works are worthy to grace these shelves, preserving the Library’s 
                    integrity for generations to come."</p>
                    <p>"All rights to the texts remain with their creators, ensuring their legacy endures. At present,
                    the Library holds [number] texts, while [number] more await the Librarian’s judgment to join this 
                    esteemed archive"</p>
                </div>

                <div class="w-52 border-1 border-game-border flex-grow-0">
                    <p class="text-center">"Game statistics"</p>
                    <p class="mb-6">"List of players in the game:"</p>

                    <ul>
                        <li><a href="#">"admin"</a>" (1)"</li>
                        <li><a href="#">"Aeris Shadowbane"</a>" (2)"</li>
                        <li><a href="#">"Kael Stormrider"</a>" (3)"</li>
                        <li><a href="#">"Lyria Moonweaver"</a>" (4)"</li>
                        <li><a href="#">"Draven Nightthorn"</a>" (5)"</li>
                        <li><a href="#">"Sylara Frostsong"</a>" (6)"</li>
                        <li><a href="#">"Thalric Ironflame"</a>" (7)"</li>
                        <li><a href="#">"Zyra Emberleaf"</a>" (8)"</li>
                        <li><a href="#">"Corwyn Duskbringer"</a>" (9)"</li>
                        <li><a href="#">"Eryndor Starforge"</a>" (10)"</li>
                        <li><a href="#">"Veyra Silverveil"</a>" (11)"</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
