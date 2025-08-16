use std::clone;
use leptos::prelude::*;
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::models::{
        album::{Album, AlbumDBModel}, 
        artist::{Artist, ArtistDBModel}, 
        song::Song
    };
    

#[component]
pub fn Controls(
    now_playing: ReadSignal<Option<Song>>
) -> impl IntoView {
    view!{
        <audio controls autoplay src = move || {
            match now_playing.get() {
                Some(song) => song.file_path,
                None => "".into()
            }
        }>
        </audio>
    }
}