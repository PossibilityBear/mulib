use crate::{
    components::{controls::controls::PlaybackState, queue::queue::SongQueueContext},
    models::song::Song,
};
use leptos::{
    ev::{self, MouseEvent},
    prelude::*,
};
use leptos_svg::svg;
use leptos_use::use_event_listener;
use stylance::import_crate_style;

import_crate_style!(song, "./src/components/song/song.module.scss");
import_crate_style!(main_style, "./src/styles/main.module.scss");

// a single song
#[component]
pub fn Song(
    song: Song,
    on_select: impl FnMut(MouseEvent) + 'static,
    is_selected: Memo<bool>,
    mut on_context: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let queue: SongQueueContext = use_context::<SongQueueContext>()
        .expect("to have found now song queue")
        .into();
    let (song, _) = signal(song);

    let song_card_ref = NodeRef::new();
    #[allow(unused_must_use)] // silence SSR warn, binds event listener on csr only
    use_event_listener(song_card_ref, ev::contextmenu, move |evt| {
        evt.prevent_default();
        on_context(evt)
    });

    let play_now = move |_| {
        _ = queue.pop_front();
        queue.push_front(song.get());
        queue.set_playback_state(PlaybackState::Play);
    };

    view! {
        <div class=move || {
                if is_selected.get() {
                    format!("{} {}", song::container, song::selected)
                } else {
                    song::container.to_string()
                }
            }
            on:click=on_select
            node_ref=song_card_ref
        >
            <div class=song::left>
                {svg!("./public/album-art-placeholder.svg", song::album_art_placeholder)}
                <div class=song::col_group>
                    <p class=song::title on:click=play_now>
                        // title
                        {move || format!("{}", song.get().title)}
                    </p>
                    <p class=song::artist>
                        // artist
                        {move || format!("{}", song.get().artist.unwrap_or_default().name)}
                    </p>
                    <p class=song::album>
                        // album
                        {move || format!("{}", song.get().album.unwrap_or_default().title)}
                    </p>
                </div>
            </div>
        </div>
    }
}
