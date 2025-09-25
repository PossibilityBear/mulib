use leptos::{prelude::{ServerFnError, *}};
use stylance::import_crate_style;
use crate::{components::song::song::{Song, SongAction}, models::{album::Album, artist::Artist, playlist::Playlist, song::Song}};
    
/// Defines the sorce of songs for the song list
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

#[server(
    prefix = "/api",
    endpoint = "get_playlist_songs"
)]
pub async fn get_playlist_songs(playlist: Playlist) -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlist_songs;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_playlist_songs(&state.db, &playlist.id).await?;

    Ok(songs)
}



/// helper function used to get around different concrete types
/// returned from different functions that implement the Future
/// trait but still have the output wrapped in a Future 
/// which is required by the Resouce
async fn song_source_helper(source: SongListSource) -> Result<Vec<Song>, ServerFnError> {
    match source {
        SongListSource::Album(_album) => todo!(),
        SongListSource::Artist(_artist) => todo!(),
        SongListSource::Playlist(playlist) => get_playlist_songs(playlist).await,
        SongListSource::All => get_all_songs().await,
    }
}

import_crate_style!(style, "./src/components/song_list/song_list.module.scss");
// a list of songs from database
#[component]
pub fn SongList (
    source: ReadSignal<SongListSource>
) -> impl IntoView {

    let songs_res = Resource::new(
        move || {
            source.get()
        },
        |source| {
            song_source_helper(source)
        }
    );

    // let (list_id, _) = signal(list_id);

    // let songs_res = Resource::new(
    //     move || {
    //         list_id.get()
    //     },
    //     |id| {get_all_songs(id)}
    // );


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