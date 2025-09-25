use leptos::prelude::*;

use crate::models::{playlist::Playlist, song::Song};

/*
TODO:
1. Add a way to create a playlist
2. Add in our add to playlist menu thing to the home page somehow
3. figure out playlist management for real.

*/




#[server(
    prefix = "/api",
    endpoint = "add_to_playlist"
)]
pub async fn add_to_playlist(playlist: Playlist, song: Song) -> Result<(), ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::add_track;

    let state = use_context::<AppState>().expect("To Have Found App State");

    _ = add_track(&state.db, &playlist.id, &song.id).await?;
    Ok(())
}

#[server(
    prefix = "/api",
    endpoint = "get_playlists"
)]
pub async fn get_playlists() -> Result<Vec<Playlist>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlists_info;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let playlists = get_playlists_info(&state.db).await?;
    Ok(playlists)
}


#[component]
pub fn add_to_playlist_menu() -> impl IntoView {
   let playlists = OnceResource::new(get_playlists()); 
   view! {
        <For each = move || {
                if let Some(Ok(playlists)) = playlists.get() {
                    playlists
                } else {
                    vec![]
                }
            }
            key = |playlist| {
                playlist.id
            }
            children = move |playlist| {
                view!{
                    <p>{ playlist.title }</p>
                }
            }
        />
    }
}
