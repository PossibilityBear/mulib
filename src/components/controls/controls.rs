use std::clone;
use leptos::{prelude::*, tachys::html::style};
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
    now_playing: ReadSignal<Option<Song>>
) -> impl IntoView {
    view!{
        <div>
            <p class=controls::now_playing> {move || {
                match now_playing.get() {
                    Some(song) => format!("Now Playing: {}", song.title),
                    None => format!("Now Playing: <None>")
                }}
            } </p>
            <audio class=main_style::centered controls autoplay src = move || {
                match now_playing.get() {
                    Some(song) => song.file_path,
                    None => "".into()
                }
            }>
            </audio>
        </div>
    }
}