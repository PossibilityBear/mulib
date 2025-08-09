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

use crate::models::song::SongDBModel;
#[server]
pub async fn get_songs(list_id: u32) -> Result<Vec<SongDBModel>, ServerFnError> {
    // use crate::database::commands::get_songs::get_songs;

    println!("Get songs called");
    use crate::database::commands::get_songs::get_songs;
    use crate::database::utils::db_connection::*;

    let mut conn = DbConnection::default(); 
    let songs = get_songs(conn);

    Ok(songs)

}


import_crate_style!(main_style, "./src/styles/main.module.scss");
// a single song
#[component] 
pub fn Song(song: SongDBModel) -> impl IntoView {
    view! {
        <p>
            {format!("Title: {}", song.title)}
        </p>
        <p>
            {format!("Author: {}", song.artist_id.unwrap_or_default())}
        </p>
        <p>
            {format!("Album: {}", song.album_id.unwrap_or_default())}
        </p>
    }
}


// a list of songs from database
#[component]
pub fn SongList (
    list_id: u32
) -> impl IntoView {
    let song_resource =  OnceResource::new(get_songs(list_id));

    let blank_song = SongDBModel {
        title: "Loading ....".to_string(),
        album_id: None,
        artist_id: None,
        id: 0,
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
                                let song_opt_vec: Vec<Option<SongDBModel>> = s
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
                        None => 0 
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