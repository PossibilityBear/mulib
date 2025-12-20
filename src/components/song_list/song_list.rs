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
use leptos_use::{OnClickOutsideOptions, on_click_outside_with_options};
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
    let refresh = RwSignal::new(false);
    let songs_res = Resource::new(
        move || (source.get(), refresh.get()),
        |(source, _)| song_source_helper(source),
    );

    let songs = RwSignal::<Vec<Song>>::new(vec![]);

    // effect here to partly decouple the songs in the list
    // from the source of the list so we can internally
    // mutate the songs without messing with the source iteslf
    Effect::new(move |_| {
        if let Some(Ok(s)) = songs_res.get() {
            songs.set(s)
        }
    });

    // vec of songs and their index to uniquely identify one song in the list
    let (selected_songs, set_selected_songs) = signal(Vec::<(usize, Song)>::new());

    let select_song = move |song_list: Vec<Song>, song: Song, index: usize| {
        let song_list = song_list.clone();
        move |ev: MouseEvent| {
            leptos::logging::log!("Selected song: {} at index: {}", song.title, index);
            if let Some(last_sel) = selected_songs.get().iter().last()
                && ev.shift_key()
            {
                // find the previously selected item if any,
                // then select throught the range
                // unwrap is safe, song can't be selected without being in song_list
                let last_sel_pos = song_list
                    .iter()
                    .position(|s| s.id == last_sel.1.id)
                    .unwrap();
                let start_pos: usize;
                let end_pos: usize;
                if last_sel_pos > index {
                    // backwards selections
                    end_pos = last_sel_pos;
                    start_pos = index;
                } else {
                    // forwards selections
                    end_pos = index;
                    start_pos = last_sel_pos;
                }
                let mut new_selections: Vec<(usize, Song)> = song_list
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(i, s)| {
                        *i >= start_pos
                            && *i <= end_pos
                            && !selected_songs.get().contains(&(*i, s.clone()))
                    })
                    .collect();
                set_selected_songs.update(|songs| songs.append(&mut new_selections));
            } else if ev.ctrl_key() {
                // if ctrl click add this song only to the list of selectiosn
                set_selected_songs.update(|songs| songs.push((index, song.clone())));
            } else {
                // if unmodified click, select only this song
                leptos::logging::log!("unmodified select");
                set_selected_songs.set(vec![(index, song.clone())]);
            }
        }
    };

    let show_context = RwSignal::new(false);
    let (context_xy, set_context_xy) = signal((0, 0));

    let on_contextmenu = move |song: Song, index: usize| {
        move |evt: MouseEvent| {
            // stop default context menu from opening
            evt.prevent_default();

            // select the right clicked song if no selection exists currently
            if selected_songs.get().iter().last().is_none() {
                leptos::logging::log!("context song: {} at index: {}", song.title, index);
                set_selected_songs.set(vec![(index, song.clone())]);
            }

            show_context.set(true);
            set_context_xy.set((evt.client_x(), evt.client_y()))
        }
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
                <button
                    on:click=move |_| {
                        refresh.set(!refresh.get());
                    }
                >
                    REFRESH
                </button>
                <SongListTitleCard source=source/>
                <Transition
                    fallback=move || view!{ <p> {"Song Loading..."} </p>}
                    >
                    <For
                        each=move || {
                            songs.get().clone().iter()
                                    .map(|song| {
                                        song.clone()
                                    })
                                    .enumerate()
                                    .collect::<Vec<(usize, Song)>>()
                        }
                        key=|(index, song)| (index.clone(), song.id)
                        children= move |(index, song)| {
                            view!{
                                <Song song=song.clone()
                                    actions={vec![SongAction::PlayNow, SongAction::AddToQueue]}
                                    on_select=select_song(songs_res.get().unwrap().unwrap(), song.clone(), index )
                                    on_context=on_contextmenu(song.clone(), index)
                                    is_selected=Memo::new(move |_| {
                                        selected_songs.get().contains(&(index, song.clone()))
                                    })
                                />
                            }
                        }
                    />
                </Transition>
            </div>
            <Show when=move || show_context.get()>
                <SongContextMenu
                    set_show=show_context
                    xy_coords=context_xy
                    node_ref=context_menu_ref
                    selected_songs=Memo::new(move |_| selected_songs.get().into_iter().map(|(_, song)| song).collect())
                    songs
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
    selected_songs: Memo<Vec<Song>>,
    songs: RwSignal<Vec<Song>>,
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
                <AddToPlaylistSubContext set_show=set_show.write_only() selected_songs songs/>
            </Show>
        </div>
    }
}

#[component]
pub fn AddToPlaylistSubContext(
    set_show: WriteSignal<bool>,
    selected_songs: Memo<Vec<Song>>,
    songs: RwSignal<Vec<Song>>,
) -> impl IntoView {
    let playlists = expect_context::<Resource<PlaylistsSource>>();
    let song_list_source = expect_context::<RwSignal<SongListSource>>();
    view! {
        <div class=style::sub_context_menu>
        <Suspense>
            {move || {
                if let Some(pls) = playlists.get() {
                    let lists = pls.lists().get();
                    lists.into_iter().map(|list| {
                        view!{
                            <button
                                class=style::context_menu
                                on:click=move |_| {
                                    if let SongListSource::Playlist(pl) = song_list_source.get()
                                        && pl.get() == list.get() {
                                            leptos::logging::log!("modified this playlist");
                                            // optimization for immediate render songs added to playlist
                                            songs.update(|s| s.append(&mut selected_songs.get()));
                                    }

                                    // update any playist that isn't currently being viewed
                                    list.update(|set_list| set_list.add_songs(selected_songs.get()));

                                    // if let SongListSource::Playlist(pl) = *song_list_source.write()
                                    //     && pl.get() == list.get() {
                                    //     // if we are updating the currently displayed playlist,
                                    //     // update the song lists source as well for immediate updates
                                    //     pl.update(|set_list| set_list.add_songs(selected_songs.get()));
                                    // } else {
                                    //     // update any playist that isn't currently being viewed
                                    //     list.update(|set_list| set_list.add_songs(selected_songs.get()));
                                    // }

                                    // list.update(|set_list| set_list.add_songs(selected_songs.get()));
                                    //
                                    // if let SongListSource::Playlist(pl) = song_list_source.get()
                                    //     && pl.get() == list.get() {
                                    //     // update the signal to force refresh if current list is
                                    //     // being viewed
                                    //     song_list_source.set(song_list_source.get());
                                    // }
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
        </Suspense>
        </div>
    }
}
