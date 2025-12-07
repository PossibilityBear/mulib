use crate::{
    components::{
        controls::controls::PlaybackState,
        queue::queue::{SongQueue, SongQueueContext},
    },
    models::song::Song,
};
use leptos::{
    ev::{self, MouseEvent},
    html::Div,
    leptos_dom::logging::console_log,
    prelude::*,
};
use leptos_svg::svg;
use leptos_use::{on_click_outside_with_options, use_event_listener, OnClickOutsideOptions};
use stylance::import_crate_style;

import_crate_style!(song, "./src/components/song/song.module.scss");
import_crate_style!(main_style, "./src/styles/main.module.scss");

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SongAction {
    AddToQueue,      // Add this song to end of queue
    PlayNow,         // plays song skipping currently playing
    RemoveFromQueue, // Removes this song from the queue (for use in queue UI)
}

// a single song
#[component]
pub fn Song(
    song: Song,
    actions: Vec<SongAction>,
    on_select: impl FnMut(MouseEvent) + 'static,
    is_selected: Memo<bool>,
    mut on_context: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let queue: SongQueue = use_context::<SongQueueContext>()
        .expect("to have found now song queue")
        .into();
    let (song, _) = signal(song);

    // let (show_context, set_show_context) = signal(false);
    // let (context_xy, set_context_xy) = signal((0, 0));

    let song_card_ref = NodeRef::new();
    use_event_listener(song_card_ref, ev::contextmenu, move |evt| {
        evt.prevent_default();
        leptos::logging::log!("Hello from local context menu event");
        on_context(evt)
    });

    // Effect::new(move |_| {
    //     // silence error from server side since this is a no-op on
    //     // server side it never gets used and that's okay.
    //     #[allow(unused_must_use)]
    //     on_click_outside_with_options(
    //         song_card_ref,
    //         move |_| {
    //             if show_context.get() {
    //                 set_show_context.set(false);
    //             }
    //         },
    //         OnClickOutsideOptions::default(), //.ignore(["#CreateDropDownButton"]),
    //     );
    // });

    let is_play_now = actions.contains(&SongAction::PlayNow);
    let play_now = move |_| {
        if is_play_now {
            _ = queue.pop_front();
            queue.push_front(song.get());
            queue.set_playback_state(PlaybackState::Play);
        }
    };

    let is_add_to_queue = actions.contains(&SongAction::AddToQueue);
    let add_to_queue = move |_| {
        if is_add_to_queue {
            console_log(&format!("adding song: {} to queue", song.get().title));
            queue.push_back(song.get());
        }
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
            <div class=song::right>
                <div class=song::actions>
                    {if is_add_to_queue {
                        Some(view! {
                            <button
                                class=main_style::svg_button
                                on:click=add_to_queue
                            >
                                {svg!("./public/add-to-queue.svg", main_style::svg_button)}
                            </button>
                        })
                    } else {
                        None
                    }}
                </div>
            </div>
        </div>
    }
}
