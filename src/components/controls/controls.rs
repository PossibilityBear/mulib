use std::{clone, collections::VecDeque};
use leptos::{ev, html, leptos_dom::logging::console_log, prelude::*, tachys::html::style};
use leptos_use::{use_event_listener, use_window};
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::models::{
        album::{Album, AlbumDBModel}, 
        artist::{Artist, ArtistDBModel}, 
        song::Song
    };
    
import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(controls, "./src/components/controls/controls.module.scss");
#[component]
pub fn Controls(
    queue: RwSignal<VecDeque<Song>>
) -> impl IntoView {

    let audio_ref = NodeRef::<html::Audio>::new();

    _ = use_event_listener(audio_ref, ev::ended, move |_| {
        console_log("Audio track has finished playing!");
        queue.update(|songs| {
            songs.pop_front();
        }) 
    });

    view!{
        <div>
            <p class=controls::now_playing> {move || {
                match queue.get().front() {
                    Some(song) => format!("Now Playing: {}", song.title),
                    None => format!("Now Playing: <None>")
                }}
            }
            </p>
            <ol>
                <For
                    each=move || queue.get()
                    key=|song| song.id 
                    children=move |song| {
                        view!{
                            <p 
                            on:click=move |_| {
                                if let Some(song_index) = queue.get().iter().position(|x| x.id == song.id) {
                                    queue.update(|queue| {
                                        queue.remove(song_index);
                                    })
                                }
                            }
                            >{song.title}</p>
                        }
                    }
                />
            </ol>
            <audio 
            node_ref=audio_ref 
            class=main_style::centered
            controls 
            autoplay 
            src = move || {
                match queue.get().front() {
                    Some(song) => song.file_path.clone(),
                    None => "".into()
                }
            }>
            </audio>
        </div>
    }
}