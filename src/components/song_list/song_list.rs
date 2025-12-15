use crate::{
    components::{
        playlist::playlist::PlaylistTitleCard,
        song::song::{Song, SongAction},
    },
    models::{
        album::Album,
        artist::Artist,
        playlist::{Playlist, PlaylistsSource},
        song::Song,
    },
};
use leptos::{
    ev::MouseEvent,
    html::Div,
    prelude::{ServerFnError, *},
};
use leptos_use::{on_click_outside_with_options, OnClickOutsideOptions};
use stylance::import_crate_style;

import_crate_style!(style, "./src/components/song_list/song_list.module.scss");

/// Defines the source of songs for the song list
#[derive(Clone, PartialEq)]
pub enum SongListSource {
    Album(Album),
    Artist(Artist),
    Playlist(RwSignal<Playlist>),
    All,
}

#[server(prefix = "/api", endpoint = "get_all_songs")]
pub async fn get_all_songs() -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::songs::get_all_songs;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_all_songs(&state.db).await?;

    Ok(songs)
}

#[server(prefix = "/api", endpoint = "get_songs_by_artist")]
pub async fn get_songs_by_artist(artist_id: i64) -> Result<Vec<Song>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::songs::get_songs_by_artist;

    let state = use_context::<AppState>().expect("To Have Found App State");

    let songs = get_songs_by_artist(&state.db, artist_id).await?;

    Ok(songs)
}

#[server(prefix = "/api", endpoint = "get_songs_by_album")]
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
        SongListSource::Playlist(playlist) => {
            playlist.get().load_songs().await.map(|songs| songs.clone())
        }
        SongListSource::All => get_all_songs().await,
    }
}

#[component]
pub fn SongListTitleCard(source: RwSignal<SongListSource>) -> impl IntoView {
    view! {
        <div class=style::SongListSourceTitleCard>
            { move || {
                match source.get() {
                    SongListSource::Album(album) => view! {<BasicListTitleCard title=album.title/>}.into_any(),
                    SongListSource::Artist(artist) => view! {<BasicListTitleCard title=artist.name/>}.into_any(),
                    SongListSource::Playlist(list) => view! {<PlaylistTitleCard playlist=list/>}.into_any(),
                    SongListSource::All => view! {<BasicListTitleCard title="All Songs".to_string()/>}.into_any(),
                }
            }}
        </div>
    }
}

#[component]
pub fn BasicListTitleCard(title: String) -> impl IntoView {
    view! {
        <h1 class=style::SongListSourceTitle>
            {title}
        </h1>
    }
}

// a list of songs from database
#[component]
pub fn SongList(source: RwSignal<SongListSource>) -> impl IntoView {
    let songs_res = Resource::new(move || source.get(), |source| song_source_helper(source));

    let (selected_songs, set_selected_songs) = signal(Vec::<Song>::new());

    let select_song = move |song_list: Vec<Song>, song: Song| {
        let song_list = song_list.clone();
        move |ev: MouseEvent| {
            if ev.shift_key() {
                leptos::logging::log!("Shift Clicked");
                // find the previously selected item if any,
                if let Some(last_sel) = selected_songs.get().iter().last() {
                    // then select throught the range
                    // unwrap is safe, song can't be selected without being in song_list
                    let last_sel_pos = song_list.iter().position(|s| s.id == last_sel.id).unwrap();
                    let cur_sel_pos = song_list.iter().position(|s| s.id == song.id).unwrap();
                    let start_pos: usize;
                    let end_pos: usize;
                    if last_sel_pos > cur_sel_pos {
                        // backwards selections
                        end_pos = last_sel_pos;
                        start_pos = cur_sel_pos;
                    } else {
                        // forwards selections
                        end_pos = cur_sel_pos;
                        start_pos = last_sel_pos;
                    }
                    let mut new_selections: Vec<Song> = song_list
                        .clone()
                        .into_iter()
                        .enumerate()
                        .filter(|(i, s)| {
                            *i >= start_pos && *i <= end_pos && !selected_songs.get().contains(s)
                        })
                        .map(|(_, s)| s)
                        .collect();
                    set_selected_songs.update(|songs| songs.append(&mut new_selections));
                } else {
                    // if none just select this song
                    set_selected_songs.set(vec![song.clone()]);
                }
            } else if ev.ctrl_key() {
                leptos::logging::log!("ctrl clicked");
                set_selected_songs.update(|songs| songs.push(song.clone()));
            } else {
                set_selected_songs.set(vec![song.clone()]);
            }

            for song in selected_songs.get() {
                leptos::logging::log!("Song name {}", song.title)
            }
        }
    };

    let show_context = RwSignal::new(false);
    let (context_xy, set_context_xy) = signal((0, 0));

    let on_contextmenu = move |evt: MouseEvent| {
        evt.prevent_default();
        leptos::logging::log!("Hello from context menu event");
        show_context.set(true);
        set_context_xy.set((evt.client_x(), evt.client_y()))
    };

    let context_menu_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        // silence error from server side since this is a no-op on
        // server side it never gets used and that's okay.
        #[allow(unused_must_use)]
        on_click_outside_with_options(
            context_menu_ref,
            move |_| {
                if show_context.get() {
                    show_context.set(false);
                }
            },
            OnClickOutsideOptions::default(), //.ignore(["#CreateDropDownButton"]),
        );
    });

    view! {
        <>
            <div class=style::songs>
                <SongListTitleCard source=source/>
                <Suspense
                    fallback=move || view!{ <p> {"Song Loading..."} </p>}
                    >
                    // TODO: Because playlists can repeat songs
                    // using song.id as the key doesn't work,
                    // will need to use a track id for playlists at least
                    <For
                        each=move || {
                            if let Some(Ok(songs)) = songs_res.get() {
                                songs.clone().iter()
                                    .map(|song| {
                                        song.clone()
                                    })
                                    .collect::<Vec<Song>>()
                            } else {
                                Vec::<Song>::new()
                            }
                        }
                        key=|song| song.id
                        children= move |song| {
                            view!{
                                <Song song=song.clone()
                                    actions={vec![SongAction::PlayNow, SongAction::AddToQueue]}
                                    on_select=select_song(songs_res.get().unwrap().unwrap(), song.clone())
                                    on_context=on_contextmenu
                                    is_selected=Memo::new(move |_| {
                                        selected_songs.get().contains(&song.clone())
                                    })
                                />
                            }
                        }
                    />
                </Suspense>
            </div>
            <Show when=move || show_context.get()>
                <SongContextMenu
                    set_show=show_context
                    xy_coords=context_xy
                    node_ref=context_menu_ref
                    selected_songs=selected_songs
                />
            </Show>
        </>
    }
}

#[component]
pub fn SongContextMenu(
    xy_coords: ReadSignal<(i32, i32)>,
    node_ref: NodeRef<Div>,
    set_show: RwSignal<bool>,
    selected_songs: ReadSignal<Vec<Song>>,
) -> impl IntoView {
    let (add_to_playlist, set_add_to_playlist) = signal(false);
    view! {
        <div class=style::context_menu
            node_ref=node_ref
            style=move || {format!("left: {}px; top: {}px;", xy_coords.get().0, xy_coords.get().1)}
        >
            <div class=style::sub_context_menu>
                <button
                    class=style::context_menu
                    on:click= move |_| {
                        set_show.set(false);
                    }
                >
                    Add to Queue
                </button>
                <button
                    class=style::context_menu
                    on:click= move |_| { set_add_to_playlist.set(true); }
                >
                    Add to Playlist
                </button>
            </div>
            <Show when=move || add_to_playlist.get()>
                <AddToPlaylistSubContext set_show=set_show.write_only() selected_songs/>
            </Show>
        </div>
    }
}

#[component]
pub fn AddToPlaylistSubContext(
    set_show: WriteSignal<bool>,
    selected_songs: ReadSignal<Vec<Song>>,
) -> impl IntoView {
    let playlists = expect_context::<Resource<PlaylistsSource>>();
    view! {
        <div class=style::sub_context_menu>
        {move || {
            if let Some(pls) = playlists.get() {
                let lists = pls.lists().get();
                lists.into_iter().map(|list| {
                    view!{
                        <button
                            class=style::context_menu
                            on:click=move |_| {
                                list.update(|set_list| set_list.add_songs(selected_songs.get()));
                                set_show.set(false);
                            }
                        >
                            {list.get().title().clone()}
                        </button>
                    }
                }).collect_view().into_any()
            } else {
                view!{}.into_any()
            }
        }}
        </div>
    }
}
