
use std::collections::VecDeque;

use leptos::prelude::*;
use stylance::import_crate_style;
use crate::components::song_list::song_list::SongList;
use crate::components::controls::controls::Controls;
use crate::components::queue::queue::{Queue, SongQueueContext};

use crate::models::song::Song;

import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(home_page, "./src/components/home_page/home_page.module.scss");
#[component]
pub fn HomePage() -> impl IntoView {
    let queue_context = SongQueueContext::default();
    let show_queue = RwSignal::<bool>::new(false);

    provide_context(queue_context);
    view! {
        <div class=home_page::container>
            <div class=home_page::main_view>
                <div class=home_page::song_list>
                    <SongList list_id=1/>
                </div>
                {move || {
                    if show_queue.get() {
                        Some(view!{<div class=home_page::queue><Queue/></div>})
                    } else {
                        None
                    }
                }}
            </div>
            <div class=home_page::controls>
                <Controls queue=queue_context.into() show_queue=show_queue />
            </div>
        </div>
        
    }
}