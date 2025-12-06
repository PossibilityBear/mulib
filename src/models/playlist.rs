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

    // /// add a song to end of the playlist
    // #[cfg(feature = "ssr")]
    // pub async fn add_song(&mut self, conn: &DbConnection, song: Song) -> Result<(), Error> {
    //     use crate::database::commands::playlists::add_track;
    //     add_track(&conn, &self.id, &song.id).await?;
    //     self.songs.push_back(song);
    //     Ok(())
    // }
    //
    // /// remove song based on position (track_num) in playlist
    // #[cfg(feature = "ssr")]
    // pub async fn remove_track(&mut self, conn: &DbConnection, track_num: u32) -> Result<(), Error> {
    //     use crate::database::commands::playlists::remove_track;
    //     remove_track(&conn, &self.id(), &(track_num as i64)).await?;
    //     self.songs.remove(track_num as usize);
    //     Ok(())
    // }
}

#[derive(Clone, Store, Serialize, Deserialize)]
pub struct PlaylistsSource2 {
    #[store(key: i64 = |list| list.id().clone())]
    lists: Vec<Playlist>,
    is_loaded: bool,
}

impl PlaylistsSource2 {
    pub fn new() -> Self {
        Self {
            lists: vec![],
            is_loaded: false,
        }
    }
}
