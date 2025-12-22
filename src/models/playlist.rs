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
    songs: RwSignal<Vec<Song>>,
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
async fn remove_track_sfn(playlist_id: i64, tracks: Vec<i64>) -> Result<(), ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::remove_tracks;

    let state = expect_context::<AppState>();
    remove_tracks(&state.db, &playlist_id, &tracks).await?;
    Ok(())
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist {
            id: i64::default(),
            title: String::new(),
            description: String::new(),
            songs: RwSignal::new(vec![]),
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
            songs: RwSignal::new(songs),
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

    /// Loads from database into this playlist can be
    /// called by a resource to get a RwSignal to the list of songs
    /// that were loaded.
    pub async fn load_songs(&self) -> RwSignal<Vec<Song>> {
        match get_songs_sfn(self.id).await {
            Ok(songs) => {
                self.songs.set(songs);
            }
            Err(e) => log!(
                "Error loading songs for playlist: {}, \n Error: \n {}",
                self.title,
                e
            ),
        }
        self.songs
    }

    /// get RwSignal to the set of songs for this playlist
    /// NOTE: this *Does Not* load the songs from the database
    /// it only returns the songs in from the in memory struct
    /// to load songs use Self.load_songs()
    pub fn get_songs(&self) -> RwSignal<Vec<Song>> {
        self.songs
    }

    /// add a song to end of the playlist.
    /// Can only be used from within a reactive context
    pub fn add_songs(&self, mut songs: Vec<Song>) {
        if songs.is_empty() {
            return;
        }

        let song_ids: Vec<i64> = songs.iter().map(|s| s.id.clone()).collect();
        let playlist_id = self.id();
        spawn_local(async move {
            if let Err(e) = add_songs_sfn(playlist_id, song_ids).await {
                log!("error adding song to playlist {}", e);
            }
        });

        // could have probably done better by implmentinging playlists
        // as a VecDequeue to have pushback method.
        self.songs.write().append(&mut songs);
    }

    /// remove song based on position (track_num) in playlist.
    /// Can only be used from within a reactive context
    pub fn remove_tracks(&self, track_nums: Vec<usize>) {
        let playlist_id = self.id();
        let mut tn_clone = track_nums.clone();
        // Ideally would have this entire method be async
        // and called from a leptos Action, but this is
        // fine for now.
        spawn_local(async move {
            if let Err(e) = remove_track_sfn(
                playlist_id,
                track_nums.into_iter().map(|n| n as i64).collect(),
            )
            .await
            {
                log!("error removing track from playlist {}", e);
            }
        });

        // Given that the track order is sorted in the song list, we can then
        // sort the track remove list and iterate just once over song list
        // removing tracks in order O(n log(n)) + O(n)) vs O(n^2) from nested
        // by element comparison
        tn_clone.sort();
        let t_start_len: usize = tn_clone.len();
        self.songs.update(|songs| {
            // copy songs so we have stable indicies to reference
            for (i, _) in songs.clone().into_iter().enumerate() {
                if let Some(remove_t) = tn_clone.first() {
                    if &i == remove_t {
                        // remove the track, adjusting index based on how many
                        // tracks have already been removed.
                        songs.remove(remove_t - (t_start_len - tn_clone.len()));
                        // update list of tracks to remove
                        tn_clone.remove(0);
                    }
                } else {
                    // no more tracks to remove break early
                    break;
                }
            }
        });
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
                log!("Error while loading playlists: {:?}", e);
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
