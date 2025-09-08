
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
    let (show_queue, set_show_queue) = signal(false);

    let toggle_queue = move |_| {
        *set_show_queue.write() = !show_queue.get();
    };
    provide_context(queue_context);
    view! {
        <div class=home_page::container>
            <h1 class=home_page::title>"Hello from mulib!"</h1>

            <div class=home_page::main_view>
                <SongList list_id=1/>
                {move || {if show_queue.get() {Some(view!{<Queue/>})} else {None}}}
            </div>
            <div class=home_page::controls>
                <Controls queue=queue_context.into()/>
                <input 
                    // class=home_page::toggle_queue
                    type="image" 
                    src={move || {if show_queue.get() {"/public/hide-queue.svg"} else {"/public/show-queue.svg"}}}
                    on:click=toggle_queue
                />
            </div>
        </div>
        
    }
}