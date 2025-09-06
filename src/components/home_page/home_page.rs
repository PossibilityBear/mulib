
use std::collections::VecDeque;

use leptos::prelude::*;
use stylance::import_crate_style;
use crate::components::song_list::song_list::SongList;
use crate::components::controls::controls::{Controls, SongQueueContext, SongQueue};
use crate::models::song::Song;

import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(home_page, "./src/components/home_page/home_page.module.scss");
#[component]
pub fn HomePage() -> impl IntoView {
    let queue_context = SongQueueContext::default();

    provide_context(queue_context);
    view! {
        <h1 class=main_style::centered>"Hello from mulib!"</h1>

        <SongList list_id=1/>
        <Controls queue=queue_context.into()/>
        
    }
}