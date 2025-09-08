use std::{clone, collections::VecDeque};
use leptos::prelude::*;
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::{components::{queue::queue::{SongQueue, SongQueueContext}, song::song::{Song, SongAction}}, models::song::Song};
    


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

    println!("hello from get songs");
    let conn = DbConnection::default(); 
    let songs = get_songs(conn);

    Ok(songs)

}

import_crate_style!(style, "./src/components/song_list/song_list.module.scss");
// a list of songs from database
#[component]
pub fn SongList (
    list_id: u32
) -> impl IntoView {
    let (list_id, set_list_id) = signal(list_id);

    let songs_res = Resource::new(
        move || {
            list_id.get()
        },
        |id| {get_songs(id)}
    );


    view! {
        <div class=style::songs>
            <Suspense
                fallback=move || view!{ <p> {"Song Loading..."} </p>}
                >
                <For 
                    each=move || {
                        if let Some(Ok(songs)) = songs_res.get() {
                            songs.clone().iter()
                                .map(|song| {
                                    Some(song.clone())
                                })
                                .collect::<Vec<Option<Song>>>()
                        } else {
                            Vec::<Option<Song>>::new()
                        }
                    }
                    key=|song| {
                        if let Some(s) = song {
                            if let Some(id) = s.id {
                                id
                            } else {
                                0
                            }   
                        } else {
                            0
                        }
                    }
                    children= move |song| {
                        view!{
                            <Song song=song actions={vec![SongAction::PlayNow, SongAction::AddToQueue]}/>
                        }
                    }

                />
            </Suspense>
        </div>
    }

}