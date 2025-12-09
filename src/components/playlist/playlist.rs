use crate::{
    components::song_list::song_list::SongListSource,
    models::playlist::{PlaylistsSource, PlaylistsSourceStoreFields},
};
use leptos::{html, prelude::*};
use leptos_svg::svg;
use reactive_stores::Store;
use stylance::import_crate_style;

import_crate_style!(playlist, "./src/components/playlist/playlist.module.scss");
import_crate_style!(main, "./src/styles/main.module.scss");

#[component]
pub fn PlaylistCard(playlist_id: i64) -> impl IntoView {
    // let song_count = list.songs.len();
    let list_source =
        use_context::<RwSignal<SongListSource>>().expect("To have found song list source context");

    let playlists =
        use_context::<Store<PlaylistsSource>>().expect("To have found playlist source context");

    //getter for making access less annoying
    let playlist = move || {
        playlists
            .lists()
            .get()
            .into_iter()
            .find(|list| list.id() == playlist_id)
            .expect("To Have found this playlist")
    };

    view! {
        <div class=move || {
            if let SongListSource::Playlist(p) = list_source.get() {
                if p.id() == playlist_id {
                    return vec![playlist::PlaylistCard, playlist::Selected].join(" ")
                }
            }
            playlist::PlaylistCard.to_string()
        }>
            <div>
                // Playlist Art
                {svg!("./public/album-art-placeholder.svg", playlist::playlist_art_placeholder)}
            </div>
            <div class=playlist::TextColGroup> <p class=playlist::PlaylistName on:click=move |_| {
                        list_source.update(|ls| {
                            *ls = SongListSource::Playlist(playlist());
                        });
                    }
                >{ move || playlist().title().clone()}</p>
            </div>
        </div>
    }
}

#[component]
pub fn PlaylistTitleCard(playlist_id: i64) -> impl IntoView {
    let (show_edit_dialog, set_show_edit_dialog) = signal(false);
    let playlists =
        use_context::<Store<PlaylistsSource>>().expect("to have found playlists source");

    let title_node: NodeRef<html::Input> = NodeRef::new();
    let desc_node: NodeRef<html::Input> = NodeRef::new();

    //getter for making access less annoying
    let playlist = move || {
        playlists
            .lists()
            .get()
            .into_iter()
            .find(|list| list.id() == playlist_id)
            .expect("To Have found this playlist")
    };

    view! {
        <>
        <div class=playlist::TitleCard>
            <div>
                <h1> {move || playlist().title().clone()} </h1>
                <p> {move || playlist().description().clone()} </p>
            </div>
            // button to open the edit dialog for playlist name, art, and description
            <button class=main::svg_button
                on:click= move |_| {
                    set_show_edit_dialog.set(true);
                }
            >
                {svg!("./public/edit.svg", main::svg_button, playlist::TitleCardEditIcon)}
            </button>
        </div>
        <Show when=move || {show_edit_dialog.get()}>
            <div class=playlist::EditDialog>
                <h1> Edit Playist </h1>
                <div class=playlist::EditDialogInputGroup>
                    <label for="title" class=playlist::InputLabel> Title: </label>
                    <input id="title"
                        node_ref=title_node
                        class=playlist::TextInput
                        value= move || playlist().title().clone()
                    />
                </div>
                <div class=playlist::EditDialogInputGroup>
                    <label for="description" class=playlist::InputLabel> Description: </label>
                    <input id="description"
                        node_ref=desc_node
                        class=playlist::TextInput
                        value= move || playlist().description().clone()
                    />
                </div>
                <div class=playlist::ExpandedGroup>
                    <div class=playlist::ButtonGroup>
                        <button
                            class=playlist::Save
                            on:click= move |_| {
                                // extract input values
                                let title_value = title_node
                                    .get()
                                    .expect("title input should be mounted")
                                    .value();
                                let desc_value = desc_node
                                    .get()
                                    .expect("description input should be mounted")
                                    .value();

                                // update playlist info
                                playlists.lists().update(|lists| {
                                    lists.iter_mut()
                                        .filter(|list| list.id() == playlist_id)
                                        .for_each(|list| {list.set_info(Some(title_value.clone()), Some(desc_value.clone()));});
                                });

                                // close dialog
                                set_show_edit_dialog.set(false);
                            }
                        >
                            Save
                        </button>
                        <button
                            class=playlist::Cancel
                            on:click= move |_| {set_show_edit_dialog.set(false);}
                        >
                            Cancel
                        </button>
                    </div>
                </div>
            </div>
        </Show>
        </>
    }
}
