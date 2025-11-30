use crate::components::controls::controls::Controls;
use crate::components::library::library::LibrarySidebar;
use crate::components::queue::queue::{Queue, SongQueueContext};
use crate::components::song_list::song_list::{SongList, SongListSource};
use crate::models::playlist::PlaylistSource;
use leptos::prelude::*;
use stylance::import_crate_style;

import_crate_style!(
    home_page,
    "./src/components/home_page/home_page.module.scss"
);
#[component]
pub fn HomePage() -> impl IntoView {
    let queue_context = SongQueueContext::default();
    let show_queue = RwSignal::<bool>::new(false);

    let list_source = RwSignal::<SongListSource>::new(SongListSource::All);
    let playlists = RwSignal::<PlaylistSource>::new(PlaylistSource::new());
    // let (list_source, _) = signal(SongListSource::Playlist(()));

    provide_context(list_source);
    provide_context(playlists);
    provide_context(queue_context);
    view! {
        <div class=home_page::container>
            <div class=home_page::main_view>
                <div class=home_page::Library>
                    <LibrarySidebar/>
                </div>
                <div class=home_page::song_list>
                    <SongList source=list_source/>
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
