use leptos::{prelude::{ServerFnError, *}};
use stylance::import_crate_style;
use crate::{components::song::song::{Song, SongAction}, models::{album::Album, artist::Artist, playlist::Playlist, song::Song}};
    
import_crate_style!(style, "./src/components/song_list/song_list.module.scss");

/// Defines the source of songs for the song list
#[derive(Clone, PartialEq)]
pub enum SongListSource {
    Album(Album),
    Artist(Artist),
    Playlist(Playlist),
    All,
}

#[server(
    prefix = "/api",
    endpoint = "get_all_songs"
)]
pub async fn get_all_songs() -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::songs::get_all_songs;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_all_songs(&state.db).await?;

    Ok(songs)
}

// There is a gotcha with server functions
// when you pass a struct that has a vec field, if the 
// vec is empty client side code omits it in the request
// but then the request fails due to missing the field
// in this case the song vec in the playlist struct can be
// empty but then the request to get playlist songs fails
// because of this, may need to make the field Option<Vec<Song>>


// the issue here is an inconsistency in how empty vecs are treated
// in the client side vs server side. workaround is just to pass
// the values you are actually using.
#[server(
    prefix = "/api",
    endpoint = "get_playlist_songs"
)]
pub async fn get_playlist_songs(playlist_id: i64) -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlist_songs;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_playlist_songs(&state.db, &playlist_id).await?;

    Ok(songs)
}

#[server(
    prefix = "/api",
    endpoint = "get_songs_by_artist"
)]
pub async fn get_songs_by_artist(artist_id: i64) -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::songs::get_songs_by_artist;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_songs_by_artist(&state.db, artist_id).await?;

    Ok(songs)
}

#[server(
    prefix = "/api",
    endpoint = "get_songs_by_album"
)]
pub async fn get_songs_by_album(album_id: i64) -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::songs::get_songs_by_album;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_songs_by_album(&state.db, album_id).await?;

    Ok(songs)
}


/// helper function used to get around different concrete types
/// returned from different functions that implement the Future
/// trait but still have the output wrapped in a Future 
/// which is required by the Resouce
async fn song_source_helper(source: SongListSource) -> Result<Vec<Song>, ServerFnError> {
    match source {
        SongListSource::Album(album) => get_songs_by_album(album.id).await, 
        SongListSource::Artist(artist) => get_songs_by_artist(artist.id).await,
        SongListSource::Playlist(playlist) => get_playlist_songs(playlist.id).await,
        SongListSource::All => get_all_songs().await,
    }
}

#[component]
pub fn SongListTitleCard(source: RwSignal<SongListSource>) -> impl IntoView {
    view!{
        <div class=style::SongListSourceTitleCard> 
            <Show when=move || {true}>
                <h1 class=style::SongListSourceTitle>
                    {match source.get() {
                        SongListSource::Album(album) => album.title,
                        SongListSource::Artist(artist) => artist.name,
                        SongListSource::Playlist(playlist) => playlist.title,
                        SongListSource::All => "All Songs".to_string(),
                    }}
                </h1>
            </Show>
        </div>
    }
}


// a list of songs from database
#[component]
pub fn SongList (
    source: RwSignal<SongListSource>
) -> impl IntoView {

    let songs_res = Resource::new(
        move || {
            source.get()
        },
        |source| {
            song_source_helper(source)
        }
    );

    view! {
        <div class=style::songs>
            <SongListTitleCard source=source/>
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
                            s.id
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