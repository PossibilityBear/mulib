
use leptos::prelude::*;
use stylance::import_crate_style;
use crate::components::song_list::song_list::SongList;
use crate::components::controls::controls::Controls;
use crate::models::song::Song;

import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(home_page, "./src/components/home_page/home_page.module.scss");
#[component]
pub fn HomePage() -> impl IntoView {
    let (song, set_song) = signal(Option::<Song>::None);

    provide_context(set_song);
    view! {
        <h1 class=main_style::centered>"Hello from mulib!"</h1>

        <SongList list_id=1/>
        <Controls now_playing=song/>
        
    }
}