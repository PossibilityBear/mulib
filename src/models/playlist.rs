use std::sync::Arc;

use crate::models::song::Song;
use leptos::{logging::log, prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A playlist is a collection of songs with some metadata
/// such as a title, description, etc...
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

impl Default for Playlist {
    fn default() -> Self {
        Playlist {
            id: i64::default(),
            title: String::new(),
            description: String::new(),
            songs: vec![],
        }
    }
}

#[server(prefix = "/api", endpoint = "get_playlist_songs")]
pub async fn get_songs_sfn(playlist_id: i64) -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlist_songs;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_playlist_songs(&state.db, &playlist_id).await?;

    Ok(songs)
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

    /// Loads songs from database into this playlist
    /// can be called from a Resource to also get the result
    /// or from an action if the goal is only to load the songs
    /// into the struct
    pub async fn load_songs<'a>(&'a mut self) -> Result<&'a Vec<Song>, ServerFnError> {
        match get_songs_sfn(self.id).await {
            Ok(songs) => {
                self.songs = songs;
                return Ok(&self.songs);
            }
            Err(e) => Err(e),
        }
    }

    /// get a reference to the set of songs for this playlist
    /// NOTE: this *Does Not* load the songs from the database
    /// it only returns the songs in from the in memory struct
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

#[server(prefix = "/api", endpoint = "get_playlists")]
pub async fn get_playlists() -> Result<Vec<Playlist>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlists_info;

    let state = use_context::<AppState>().expect("To have Found App State");

    let playlists = get_playlists_info(&state.db).await?;

    Ok(playlists)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlaylistsSource {
    lists: Arc<RwSignal<Vec<RwSignal<Playlist>>>>,
}
impl PlaylistsSource {
    // asycn initialization to be setup inside a resouce to take advantage
    // of <Suspense/> component breaks durring SSR to load playlists from DB
    pub async fn new() -> Self {
        let playlists_res = get_playlists().await;

        match playlists_res {
            Ok(lists) => Self {
                lists: Arc::new(RwSignal::new(
                    lists.into_iter().map(|list| RwSignal::new(list)).collect(),
                )),
            },
            Err(e) => {
                leptos::logging::log!("Error while loading playlists: {:?}", e);
                Self {
                    lists: Arc::new(RwSignal::new(vec![])),
                }
            }
        }
    }

    /// creates a new playlist and writes the new playlist to provided signal
    pub fn new_playlist(&mut self, new_playlist: RwSignal<Playlist>) {
        let new_pl_res = OnceResource::new(create_playlist());
        let lists = Arc::clone(&self.lists);
        Effect::new(move |_| {
            if let Some(Ok(pl)) = new_pl_res.get() {
                new_playlist.set(pl.clone());
                lists.write().push(new_playlist);
            }
        });
    }

    /// Get the set of playlists from this source
    pub fn lists(&self) -> RwSignal<Vec<RwSignal<Playlist>>> {
        *self.lists
    }

    /// Get a single playlist from this source
    pub fn list(&self, playlist_id: &i64) -> RwSignal<Playlist> {
        (*self.lists)
            .get()
            .into_iter()
            .find(|l| &l.get().id() == playlist_id)
            .unwrap()
    }
}
