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
async fn song_source_helper(source: SongListSource) -> RwSignal<Vec<Song>> {
    match source {
        SongListSource::Album(album) => RwSignal::new(get_songs_by_album(album.id).await.unwrap()),
        SongListSource::Artist(artist) => {
            RwSignal::new(get_songs_by_artist(artist.id).await.unwrap())
        }
        SongListSource::Playlist(playlist) => playlist.get().load_songs().await,
        SongListSource::All => RwSignal::new(get_all_songs().await.unwrap()),
    }
}

/// The title card for a list of songs
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

/// A title card containing just a title
#[component]
pub fn BasicListTitleCard(title: String) -> impl IntoView {
    view! {
        <h1 class=style::SongListSourceTitle>
            {title}
        </h1>
    }
}

/// a list of songs from loaded from a SongListSource
#[component]
pub fn SongList(source: RwSignal<SongListSource>) -> impl IntoView {
    // NOTE: currently the order songs are returned from the database
    // is coupled to the order that they are displayed in the UI
    // if alternate sorting is implemented we would need to decouple
    // so each song would have a UI sort index and a databse track number

    // load the songs from the source
    let songs_res = Resource::new(move || source.get(), |source| song_source_helper(source));

    /////////////////////////// Selection /////////////////////////////////////
    // vec of songs and their index to uniquely identify one song in the list
    let (selected_songs, set_selected_songs) = signal(Vec::<(usize, Song)>::new());
    // effect to erase selection when song list source changes.
    Effect::new(move |_| {
        // bind effect to song list source
        let _ = source.get();
        // on change clear selection
        set_selected_songs.set(vec![]);
    });

    // handler for click / selection of songs in the list
    let select_song = move |songs: RwSignal<Vec<Song>>, song: Song, index: usize| {
        move |ev: MouseEvent| {
            if let Some((last_sel_pos, _last_sel)) = selected_songs.get().iter().last()
                && ev.shift_key()
            {
                // find the previously selected item if any,
                // then select throught the range
                // unwrap is safe, song can't be selected without being in song_list
                let start_pos: usize;
                let end_pos: usize;
                if *last_sel_pos > index {
                    // backwards selections
                    end_pos = *last_sel_pos;
                    start_pos = index;
                } else {
                    // forwards selections
                    end_pos = index;
                    start_pos = *last_sel_pos;
                }
                let mut new_selections: Vec<(usize, Song)> = songs
                    .get()
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
                set_selected_songs.set(vec![(index, song.clone())]);
            }
        }
    };

    ///////////////////////////// Context Menu ////////////////////////////////
    let show_context = RwSignal::new(false);
    let (context_xy, set_context_xy) = signal((0, 0));

    // handler for selection context menu
    let on_contextmenu = move |song: Song, index: usize| {
        move |evt: MouseEvent| {
            // stop default context menu from opening
            evt.prevent_default();

            // override selection with new song if the song isn't
            // in the current selection
            if !selected_songs.get().contains(&(index, song.clone())) {
                set_selected_songs.set(vec![(index, song.clone())]);
            }

            show_context.set(true);
            set_context_xy.set((evt.client_x(), evt.client_y()))
        }
    };

    // effect to close context menu on click outside
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

    /////////////////////// View Definition ///////////////////////////////////
    view! {
        <>
            <div class=style::songs>
                <SongListTitleCard source=source/>
                <Transition
                    fallback=move || view!{ <p> {"Song Loading..."} </p>}
                    >
                    <For
                        each=move || {
                            if let Some(rw_songs) = songs_res.get() {
                                rw_songs.get().into_iter()
                                        .enumerate()
                                        .collect::<Vec<(usize, Song)>>()
                            } else {
                                vec![]
                            }
                        }
                        key=|(index, song)| (index.clone(), song.id)
                        children= move |(index, song)| {
                            view!{
                                <Song song=song.clone()
                                    actions={vec![SongAction::PlayNow, SongAction::AddToQueue]}
                                    on_select=select_song(songs_res.get().unwrap(), song.clone(), index )
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
                    selected_songs=selected_songs.get()
                    source
                />
            </Show>
        </>
    }
}

/// Context menu with a list of actions to be taken on a set of selected songs
#[component]
fn SongContextMenu(
    xy_coords: ReadSignal<(i32, i32)>,
    node_ref: NodeRef<Div>,
    set_show: RwSignal<bool>,
    selected_songs: Vec<(usize, Song)>,
    source: RwSignal<SongListSource>,
) -> impl IntoView {
    // visibility controls for playlist selection sub context menu
    let (is_add, set_is_add) = signal(false);

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
                    on:click= move |_| {
                        // expose sub-context menu for playlist selection
                        set_is_add.set(true);
                    }
                >
                    Add to Playlist
                </button>
                {
                let selected_songs = selected_songs.clone();
                move || {
                    if let SongListSource::Playlist(pl)= source.get() {
                        let track_nums: Vec<usize> = selected_songs
                            .clone()
                            .into_iter()
                            .map(|(i, _)| i)
                            .collect();
                        view!{
                            <button
                                class=style::context_menu
                                on:click= move |_| {
                                    pl.update(|pl| {
                                        pl.remove_tracks(track_nums.clone());
                                    });
                                    set_show.set(false);
                                }
                            >
                                Remove from Playlist
                            </button>
                        }.into_any()
                    } else {
                        view!{}.into_any()
                    }
                } }
            </div>
            <Show when=move || is_add.get()>
                <AddToPlaylistSubContext set_show=set_show.write_only() selected_songs={
                    selected_songs
                        .clone()
                        .into_iter()
                        .map(|(_, s)| s)
                        .collect()
                }/>
            </Show>
        </div>
    }
}

/// Context sub menu for a list of playlists actions can be applied to
#[component]
pub fn AddToPlaylistSubContext(
    set_show: WriteSignal<bool>,
    selected_songs: Vec<Song>,
) -> impl IntoView {
    let playlists = expect_context::<Resource<PlaylistsSource>>();
    view! {
        <div class=style::sub_context_menu>
        <Suspense>
            {move || {
                if let Some(pls) = playlists.get() {
                    let lists = pls.lists().get();
                    lists.into_iter().map(|list| {
                        let selection = selected_songs.clone();
                        view!{
                            <button
                                class=style::context_menu
                                on:click=move |_| {
                                    list.update(|set_list| set_list.add_songs(selection.clone()));
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
