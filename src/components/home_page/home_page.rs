use crate::components::controls::controls::Controls;
use crate::components::library::library::LibrarySidebar;
use crate::components::queue::queue::{Queue, SongQueueContext};
use crate::components::song_list::song_list::{SongListSource, SongListView};
use crate::models::playlist::PlaylistsSource;
use leptos::prelude::*;
use stylance::import_crate_style;

import_crate_style!(
    home_page,
    "./src/components/home_page/home_page.module.scss"
);

#[component]
pub fn HomePage() -> impl IntoView {
    let show_queue = RwSignal::<bool>::new(false);

    let queue_context = SongQueueContext::default();
    provide_context(queue_context);

    let list_source = RwSignal::new(SongListSource::All);
    provide_context(list_source);

    // define playlists context
    let playlists_res = Resource::new(|| {}, |_| PlaylistsSource::new());
    provide_context(playlists_res);

    view! {
        <div class=home_page::container>
            <div class=home_page::main_view>
                <div class=home_page::Library>
                    <LibrarySidebar/>
                </div>
                <div class=home_page::song_list>
                    <SongListView source=list_source/>
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
