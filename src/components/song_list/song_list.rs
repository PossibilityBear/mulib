use std::{clone, collections::VecDeque};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use std::ops::Range;
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::{components::controls::controls::{PlaybackState, SongQueue, SongQueueContext}, models::{
        album::{Album, AlbumDBModel}, 
        artist::{Artist, ArtistDBModel}, 
        song::Song
    }};
    


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

#[server]
pub async fn get_song_count(list_id: u32) -> Result<usize, ServerFnError> {
    use crate::database::utils::db_connection::*;

    println!("hello from get songs");
    let conn = DbConnection::default(); 

    let db = conn.db();
    let mut stmt = db.prepare(
        "
        SELECT 
            Count(*) AS SongCount
        FROM Song AS s
        "
    ).unwrap();

    let song_counts = stmt.query_map([], |row| {
        let count: usize = row.get(0)?;
        Ok(count)
    }).unwrap();

    let count: usize = *song_counts
        .map(|count| count.expect("to have gotten count"))
        .collect::<Vec<usize>>()
        .first().unwrap_or(&0); 

    Ok(count)

}


import_crate_style!(style, "./src/components/song_list/song_list.module.scss");
import_crate_style!(main_style, "./src/styles/main.module.scss");
// a single song
#[component] 
pub fn Song(song: Option<Song>) -> impl IntoView {
    let queue: SongQueue = use_context::<SongQueueContext>().expect("to have found now song queue").into();
    let (song, _) = signal(song);
    view! {
        <Show
            when=move || {song.get().is_some()}
            fallback=|| view!{<td>{"loading..."}</td>}
            >
            <td>
                {format!("{}", song.get().expect("some song").title)}
            </td>
            <td>
                {format!("{}", song.get().expect("some song").artist.unwrap_or_default().name)}
            </td>
            <td>
                {format!("{}", song.get().expect("some song").album.unwrap_or_default().title)}
            </td>
            <td class=style::button_col>
                <input 
                    type="image"
                    src="/public/play.svg"
                    class=style::song_play_button
                    on:click= move |_| {
                        console_log(&format!("Clicked play on {}", song.get().expect("some song").title));
                        _ = queue.pop_front();
                        queue.push_front(song.get().expect("to find song"));
                        queue.set_playback_state(PlaybackState::Play);
                }/>
            </td>
            <td class=style::button_col>
                <input 
                    type="image"
                    src="/public/add-to-queue.svg"
                    class=style::song_play_button
                    on:click= move |_| {
                        queue.push_back(song.get().expect("to find song"));
                }/>
            </td>
        </Show>
    }
}
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
            <table class=style::songs>
                <thead>
                    <tr>
                        <th>{"Title"}</th>
                        <th>{"Author"}</th>
                        <th>{"Album"}</th>
                        <th class=style::button_col>{""}</th>//play button
                        <th class=style::button_col>{""}</th>//add to queue button
                    </tr>
                </thead>
                <tbody>
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
                                <tr>
                                    <Song song=song/>
                                </tr>
                            }
                        }

                    />
                </tbody>
            </table>
            </Suspense>
        </div>
    }

}