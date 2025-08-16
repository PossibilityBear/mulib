use std::clone;
use leptos::prelude::*;
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::models::{
        album::{Album, AlbumDBModel}, 
        artist::{Artist, ArtistDBModel}, 
        song::Song
    };
    


#[derive(Serialize, Deserialize, Clone)]
pub struct SongData {
    title: String,
    author: String,
    album: String,
    song_id: u32,
}
impl Default for SongData {
    fn default() -> Self {
        Self { 
            title: "loading....".to_string(), 
            author: "loading....".to_string(), 
            album:  "loading....".to_string(), 
            song_id: 0,
        }
    }
}

#[server]
pub async fn get_songs(list_id: u32) -> Result<Vec<Song>, ServerFnError> {
    use crate::database::commands::get_songs::get_songs;
    use crate::database::utils::db_connection::*;

    let conn = DbConnection::default(); 
    let songs = get_songs(conn);

    Ok(songs)

}


import_crate_style!(main_style, "./src/styles/main.module.scss");
// a single song
#[component] 
pub fn Song(song: Song) -> impl IntoView {
    let now_playing = use_context::<WriteSignal<Option<Song>>>().expect("to have found now playing song");
    let song_copy = song.clone();
    view! {
        <p>
            {format!("Title: {}", song.title)}
        </p>
        <p>
            {format!("Author: {}", song.artist.unwrap_or_default().name)}
        </p>
        <p>
            {format!("Album: {}", song.album.unwrap_or_default().title)}
        </p>
        <button on:click= move |_|{
            *now_playing.write() = Some(song_copy.clone());
        }>{"play"}</button>
    }
}


// a list of songs from database
#[component]
pub fn SongList (
    list_id: u32
) -> impl IntoView {
    let song_resource =  OnceResource::new(get_songs(list_id));

    let blank_song = Song {
        title: "Loading ....".to_string(),
        album: None,
        artist: None,
        id: Some(0),
        file_path: "loading....".to_string(),

    };


    view! {
        <Suspense
            fallback=move || view!{<p> "Loading..." </p>}
        >
            <ul>
                <For 
                    each=move || { 
                        match song_resource.get() {
                            Some(Ok(s)) => {
                                let song_opt_vec: Vec<Option<Song>> = s
                                    .into_iter()
                                    .map(|val| Some(val))
                                    .collect();
                                song_opt_vec
                            },
                            Some(Err(e)) => {dbg!(e); vec!{None}},
                            None => vec!{None},
                        }
                    }
                    key=|song| {match song {
                        Some(s) => s.id,
                        None => Some(0) 
                    }}
                    children=move |song| {
                        match song {
                            Some(s) => 
                                view!{ <li><Song song=s.clone()/></li>},
                            None => view!{<li><Song song=blank_song.clone()/></li>},
                        }
                    }
                />
            </ul>
        </Suspense>
    }
}