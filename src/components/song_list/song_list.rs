use leptos::prelude::*;
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::{components::song::song::{Song, SongAction}, models::song::Song};
    


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

#[server(
    prefix = "/api",
    endpoint = "get_all_songs"
)]
pub async fn get_all_songs(_list_id: u32) -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::songs::get_all_songs;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_all_songs(&state.db).await?;

    Ok(songs)
}

import_crate_style!(style, "./src/components/song_list/song_list.module.scss");
// a list of songs from database
#[component]
pub fn SongList (
    list_id: u32
) -> impl IntoView {
    let (list_id, _) = signal(list_id);

    let songs_res = Resource::new(
        move || {
            list_id.get()
        },
        |id| {get_all_songs(id)}
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