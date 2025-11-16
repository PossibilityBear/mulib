use crate::components::album::album::AlbumCard;
use crate::components::artist::artist::ArtistCard;
use crate::components::playlist::playlist::PlaylistCard;
use crate::components::song_list::song_list::SongListSource;
use crate::models::album::Album;
use crate::models::artist::Artist;
use crate::models::playlist::Playlist;
use leptos::prelude::*;
use leptos_svg::svg;
use stylance::import_crate_style;

import_crate_style!(library, "./src/components/library/library.module.scss");
import_crate_style!(main, "./src/styles/main.module.scss");

#[derive(Clone, PartialEq, Copy)]
pub enum Tabs {
    Artists,
    Albums,
    Playlists,
}

#[component]
pub fn TabSelector(tab: Tabs, tab_selection: RwSignal<Tabs>) -> impl IntoView {
    let tab_name = match tab {
        Tabs::Artists => "Artists",
        Tabs::Albums => "Albums",
        Tabs::Playlists => "Playlists",
    };

    view! {
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

#[server(prefix = "/api", endpoint = "get_playlists")]
pub async fn get_playlists() -> Result<Vec<Playlist>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlists_info;

    let state = use_context::<AppState>().expect("To have Found App State");

    let playlists = get_playlists_info(&state.db).await?;

    Ok(playlists)
}

#[server(prefix = "/api", endpoint = "get_artists")]
pub async fn get_artists() -> Result<Vec<Artist>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::artists::get_all_artists;

    let state = use_context::<AppState>().expect("To have Found App State");

    let playlists = get_all_artists(&state.db).await?;

    Ok(playlists)
}

#[server(prefix = "/api", endpoint = "get_albums")]
pub async fn get_albums() -> Result<Vec<Album>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::albums::get_all_albums;

    let state = use_context::<AppState>().expect("To have Found App State");

    let playlists = get_all_albums(&state.db).await?;

    Ok(playlists)
}

#[component]
pub fn CreateDropDown() -> impl IntoView {
    let (show_dd, set_show_dd) = signal(false);

    view! {
        <div class=library::CreateDropDown>
            <button
                class=library::CreateDropDown
                on:click=move |_| {
                    set_show_dd.set(!show_dd.get());
                }
            >
                {svg!("./public/plus.svg", main::svg_button, library::CreateIcon)}
            </button>
            <Show when=move || {show_dd.get()}>
                <div class=library::CreateDropDownOpts>
                    <button
                        class=library::CreateDropDownOpt
                        on:click=move |_| {
                            set_show_dd.set(!show_dd.get());
                            // Create a new Playlist and navigate to it
                        }
                    >
                        New Playlist
                    </button>
                    <button
                        class=library::CreateDropDownOpt
                        on:click=move |_| {
                            set_show_dd.set(!show_dd.get());
                            // Open Upload dialog
                        }
                    >
                        Upload Music
                    </button>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn LibrarySidebar() -> impl IntoView {
    let tab_selection = RwSignal::new(Tabs::Playlists);

    let list_source =
        use_context::<RwSignal<SongListSource>>().expect("To have found song list source context");

    view! {
        <div class=library::LibContainer>
            <div class=library::HeaderRow>
                <h1 class=library::Title> Library </h1>
                <CreateDropDown/>
                <button class=library::AllSongs
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
            <div class=library::ListContainer>
                <ArtistList tab_selection=tab_selection.read_only()/>
                <AlbumList tab_selection=tab_selection.read_only()/>
                <PlaylistList tab_selection=tab_selection.read_only()/>

                // For some reason trying to control visibility
                // of components with For gets weird non-fatal errors
                // instead just pass the signal to each component
                // for it to decide if it should render.

                // Now there is an issue where it doesn't reload
                // library list ever.

                // {move || {match tab_selection.get() {
                //     Tabs::Playlists => view!{ <AlbumList/> }.into_any(),
                //     Tabs::Albums => view!{ <AlbumList/> }.into_any(),
                //     Tabs::Artists => view!{ <AlbumList/> }.into_any(),
                // }}}
            </div>
        </div>
    }
}

#[component]
pub fn PlaylistList(tab_selection: ReadSignal<Tabs>) -> impl IntoView {
    let playlists = OnceResource::new(get_playlists());

    view! {
        <Show when=move || tab_selection.get() == Tabs::Playlists>
        <Suspense
            fallback=move || view!{<p> {"loading..."}</p>}
        >
            <For
                each=move || {
                    if let Some(Ok(playlists)) = playlists.get() {
                        playlists.into_iter().map(|playlist| {
                            playlist
                        })
                        .collect::<Vec<Playlist>>()
                    } else {
                        Vec::<Playlist>::new()
                    }
                }
                key=|playlist| playlist.id
                children=move |playlist| {
                    view!{
                        <PlaylistCard list=playlist/>
                    }
                }
            />
        </Suspense>
        </Show>
    }
}

#[component]
pub fn ArtistList(tab_selection: ReadSignal<Tabs>) -> impl IntoView {
    let artists = OnceResource::new(get_artists());

    view! {
        <Show when=move || tab_selection.get() == Tabs::Artists>
        <Suspense
            fallback=move || view!{<p> {"loading..."}</p>}
        >
            <For
                each=move || {
                    if let Some(Ok(artists)) = artists.get() {
                        artists.into_iter().map(|artist| {
                            artist
                        })
                        .collect::<Vec<Artist>>()
                    } else {
                        Vec::<Artist>::new()
                    }
                }
                key=|artist| {artist.id}
                children=move |artist| {
                    view!{
                        <ArtistCard artist=artist/>
                    }
                }
            />
        </Suspense>
        </Show>
    }
}

#[component]
pub fn AlbumList(tab_selection: ReadSignal<Tabs>) -> impl IntoView {
    let albums = OnceResource::new(get_albums());

    view! {
        <Show when=move || tab_selection.get() == Tabs::Albums>
        <Suspense
            fallback=move || view!{<p> {"loading..."}</p>}
        >
            <For
                each=move || {
                    if let Some(Ok(albums)) = albums.get() {
                        albums.into_iter().map(|album| {
                            album
                        })
                        .collect::<Vec<Album>>()
                    } else {
                        Vec::<Album>::new()
                    }
                }
                key=|album| {album.id}
                children=move |album| {
                    view!{
                        <AlbumCard album=album/>
                    }
                }
            />
        </Suspense>
        </Show>
    }
}
