use leptos::prelude::*;
use stylance::import_crate_style;
use crate::components::song_list::song_list::SongListSource;
use crate::models::playlist::Playlist;
use crate::components::playlist::playlist::PlaylistCard;

import_crate_style!(library, "./src/components/library/library.module.scss");
import_crate_style!(main, "./src/styles/main.module.scss");

#[derive(Clone, PartialEq, Copy)]
pub enum Tabs {
    Artists,
    Albums,
    Playlists
}


#[component] 
pub fn TabSelector(tab: Tabs, tab_selection: RwSignal<Tabs>) -> impl IntoView {
    let tab_name = match tab {
        Tabs::Artists => "Artists",
        Tabs::Albums => "Albums",
        Tabs::Playlists => "Playlists",
    };

    view!{
        <button 
            class=move || {
                if tab_selection.get() == tab {
                    vec![library::Tab, library::TabSelected].join(" ")
                } else {
                    library::Tab.to_string()
                }
            }
            on:click=move |_| {
                tab_selection.set(tab);
            }
        > {tab_name} </button>
    }
}

#[server]
pub async fn get_playlists() -> Result<Vec<Playlist>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlists_info;

    let state = use_context::<AppState>().expect("To have Found App State");

    let playlists = get_playlists_info(&state.db).await?;

    Ok(playlists)
}


#[component]
pub fn LibrarySidebar() -> impl IntoView {
    let tab_selection = RwSignal::new(Tabs::Playlists);

    let list_source = use_context::<RwSignal<SongListSource>>().expect("To have found song list source context");

    let playlists = Resource::new(
        move || {
            // source.get()
        },
        |_| {
            get_playlists()
        }
    );

    view!{
        <div class=library::LibContainer>
            <div class=library::HeaderRow>
                <h1 class=library::Title> Library </h1>
                <button class=library::Tab
                    on:click=move |_| {
                        list_source.set(SongListSource::All);
                    }
                >All Songs</button>
            </div>
            <div class=library::HeaderRow>
                <TabSelector tab=Tabs::Artists tab_selection=tab_selection/>
                <TabSelector tab=Tabs::Albums tab_selection=tab_selection/>
                <TabSelector tab=Tabs::Playlists tab_selection=tab_selection/>
            </div>
            <Suspense
                fallback=move || view!{<p> {"loading..."}</p>}
            >
                <For 
                    each=move || {
                        if let Some(Ok(playlists)) = playlists.get() {
                            playlists.into_iter().map(|playlist| {
                                Some(playlist)
                            })
                            .collect::<Vec<Option<Playlist>>>()
                        } else {
                            Vec::<Option<Playlist>>::new()
                        }
                    }
                    key=|playlist| {
                        if let Some(p) = playlist {
                            p.id
                        } else {
                            0
                        }
                    }
                    children=move |playlist| {
                        view!{
                            <PlaylistCard list=playlist/>
                        }
                    }
                />
            </Suspense>
        </div>
    }
}