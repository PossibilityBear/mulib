use std::collections::HashMap;

use crate::models::song::Song;
use leptos::{logging::log, prelude::*, reactive::spawn_local};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Playlist {
    id: i64,
    title: String,
    description: String,
    songs: Vec<Song>,
}

#[server(prefix = "/api", endpoint = "set_playlist_info")]
async fn set_info_sfn(
    playlist_id: i64,
    title: String,
    description: String,
) -> Result<(), ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::update_info;
    let state = use_context::<AppState>().expect("to have found AppState");
    update_info(&state.db, &playlist_id, &title, &description).await?;
    Ok(())
}

#[server]
async fn add_songs_sfn(playlist_id: i64, song_ids: Vec<i64>) -> Result<(), ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::add_tracks;

    let state = expect_context::<AppState>();
    add_tracks(&state.db, &playlist_id, &song_ids).await?;
    Ok(())
}

#[server]
async fn remove_track_sfn(playlist_id: i64, track: i64) -> Result<(), ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::remove_track;

    let state = expect_context::<AppState>();
    remove_track(&state.db, &playlist_id, &track).await?;
    Ok(())
}

impl Playlist {
    /// create a new playlist
    pub fn new(id: i64, title: String, description: String, songs: Vec<Song>) -> Self {
        Playlist {
            id,
            title,
            description,
            songs,
        }
    }

    /// get the id for this playlist
    pub fn id(&self) -> i64 {
        self.id
    }

    /// get a reference to the title for the playlist
    pub fn title<'a>(&'a self) -> &'a String {
        &self.title
    }

    /// get a reference to the description for the playlist
    pub fn description<'a>(&'a self) -> &'a String {
        &self.description
    }

    /// udpate the title and description of the playlist
    /// if None is provided for title or description that field
    /// will be left unchanged
    pub fn set_info(&mut self, title: Option<String>, description: Option<String>) {
        description.map(|desc| self.description = desc);
        title.map(|title| self.title = title);
        let id = self.id.clone();
        let title = self.title.clone();
        let desc = self.description.clone();
        spawn_local(async move {
            let res = set_info_sfn(id, title, desc).await;
            if let Err(e) = res {
                log!("Error setting playlist info: {}", e);
            }
        });
    }

    /// get a reference to the set of songs for this playlist
    pub fn get_songs<'a>(&'a self) -> &'a Vec<Song> {
        &self.songs
    }

    /// add a song to end of the playlist.
    /// Can only be used from within a reactive context
    pub fn add_songs(&mut self, mut songs: Vec<Song>) {
        if songs.is_empty() {
            return;
        }

        let song_ids: Vec<i64> = songs.iter().map(|s| s.id.clone()).collect();
        let playlist_id = self.id();
        spawn_local(async move {
            if let Err(e) = add_songs_sfn(playlist_id, song_ids).await {
                leptos::logging::log!("error adding song to playlist {}", e);
            }
        });

        // could have probably done better by implmentinging playlists
        // as a VecDequeue to have pushback method.
        self.songs.append(&mut songs);
    }

    /// remove song based on position (track_num) in playlist.
    /// Can only be used from within a reactive context
    pub async fn remove_track(&mut self, track_num: u32) {
        let playlist_id = self.id();
        spawn_local(async move {
            if let Err(e) = remove_track_sfn(playlist_id, track_num as i64).await {
                leptos::logging::log!("error removing track from playlist {}", e);
            }
        });

        self.songs.remove(track_num as usize);
    }
}

#[server(prefix = "/api", endpoint = "create_new_playlist")]
pub async fn create_playlist() -> Result<Playlist, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::create_playlist;

    let state = use_context::<AppState>().expect("To have Found App State");

    let playlist = create_playlist(&state.db).await?;

    Ok(playlist)
}

#[derive(Clone, Store, Serialize, Deserialize)]
pub struct PlaylistsSource {
    #[store(key: i64 = |list| list.id().clone())]
    lists: Vec<Playlist>,
    is_loaded: bool,
}

impl PlaylistsSource {
    pub fn new() -> Self {
        Self {
            lists: vec![],
            is_loaded: false,
        }
    }

    /// creates a new playlist and optionally writes playlist
    /// to as provided signal
    pub fn new_playlist(&mut self, new_playlist: RwSignal<Option<Playlist>>) {
        let new_pl_res = OnceResource::new(create_playlist());
        // TODO: get a method where I can modify self.lists
        // from within an effect using a smart pointer of some kind
        // Effect::new(move |_| {
        //     if let Some(Ok(pl)) = new_pl_res.get() {
        //         new_playlist.set(Some(pl.clone()));
        //         self.lists.push(pl);
        //     }
        // });
    }
}
