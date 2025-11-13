use leptos::{leptos_dom::logging::console_log, prelude::*};
use stylance::import_crate_style;
use crate::{components::{controls::controls::PlaybackState, queue::queue::{SongQueue, SongQueueContext}}, models::song::Song};
    


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SongAction {
    AddToQueue, // Add this song to end of queue
    PlayNow, // plays song skipping currently playing
    RemoveFromQueue, // Removes this song from the queue (for use in queue UI)
}

import_crate_style!(song, "./src/components/song/song.module.scss");
// import_crate_style!(main_style, "./src/styles/main.module.scss");
// a single song
#[component] 
pub fn Song(song: Song, actions: Vec<SongAction>) -> impl IntoView {
    let queue: SongQueue = use_context::<SongQueueContext>().expect("to have found now song queue").into();
    let (song, _) = signal(song);


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

    // let is_remove_from_queue = actions.contains(&SongAction::RemoveFromQueue);
    // let remove_from_queue = move |_| {
    //     if is_remove_from_queue {
    //         queue.remove_songs(song.get().expect("to find song").id.expect("to have Id"));
    //     }
    // }
    view! {
        <div class=song::container >
            <div class=song::left>
                <img class=song::album_art src="/public/album-art-placeholder.svg"/>
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
                            <input class=song::button type="image" src="/public/add-to-queue.svg" on:click=add_to_queue/>
                        })
                    } else {
                        None
                    // {if is_remove_from_queue{
                    //     Some(view! {
                    //         <input class=song::button type="image" src="/public/add-to-queue.svg" on:click=add_to_queue/>
                    //     })
                    // } else {
                    //     None
                    // }}
                    }}
                </div>
            </div>
        </div>
    }
}