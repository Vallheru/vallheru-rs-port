use leptos::prelude::*;

#[component]
pub fn LibraryMain() -> impl IntoView {
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
