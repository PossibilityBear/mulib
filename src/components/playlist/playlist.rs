use crate::{components::song_list::song_list::SongListSource, models::playlist::Playlist};
use leptos::{html, prelude::*};
use leptos_svg::svg;
use stylance::import_crate_style;

import_crate_style!(playlist, "./src/components/playlist/playlist.module.scss");
import_crate_style!(main, "./src/styles/main.module.scss");

#[component]
pub fn PlaylistCard(list: Playlist) -> impl IntoView {
    // let song_count = list.songs.len();
    let list_source =
        use_context::<RwSignal<SongListSource>>().expect("To have found song list source context");

    let (list, _) = signal(list);
    view! {
        <div class=move || {
            if let SongListSource::Playlist(p) = list_source.get() {
                if p.id() == list.get().id() {
                    return vec![playlist::PlaylistCard, playlist::Selected].join(" ")
                }
            }
            playlist::PlaylistCard.to_string()
        }>
            <div>
                // Playlist Art
                {svg!("./public/album-art-placeholder.svg", playlist::playlist_art_placeholder)}
            </div>
            <div class=playlist::TextColGroup>
                <p class=playlist::PlaylistName
                    on:click=move |_| {
                        list_source.set(SongListSource::Playlist(list.get()))
                    }
                >{ move || list.get().title().clone()}</p>
                // <p class=playlist::SongCount>{song_count} songs</p>
            </div>
        </div>
    }
}

#[component]
pub fn PlaylistTitleCard(playlist: Playlist) -> impl IntoView {
    let (show_edit_dialog, set_show_edit_dialog) = signal(false);
    let playlist = RwSignal::new(playlist);
    view! {
        <>
        <div class=playlist::TitleCard>
            <div>
                <h1> {move || playlist.get().title().clone()} </h1>
                <p> {move || playlist.get().description().clone()} </p>
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
            <PlaylistEditDialog playlist show=set_show_edit_dialog/>
        </Show>
        </>
    }
}

#[component]
pub fn PlaylistEditDialog(playlist: RwSignal<Playlist>, show: WriteSignal<bool>) -> impl IntoView {
    let title_node: NodeRef<html::Input> = NodeRef::new();

    view! {
        <div class=playlist::EditDialog>
            <h1> Edit Playist </h1>
            <div class=playlist::EditDialogInputGroup>
                <label for="title" class=playlist::InputLabel> Title: </label>
                <input id="title"
                    node_ref=title_node
                    class=playlist::TextInput
                    value= move || playlist.get().title().clone()
                />
            </div>
            <div class=playlist::EditDialogInputGroup>
                <label for="description" class=playlist::InputLabel> Description: </label>
                <input id="description"
                    class=playlist::TextInput
                    value= move || playlist.get().description().clone()
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

                            // update playlist info
                            playlist.update(|pl| {
                                pl.set_info(Some(title_value), None);
                            });


                            // close dialog
                            show.set(false);
                        }
                    >
                        Save
                    </button>
                    <button
                        class=playlist::Cancel
                        on:click= move |_| {show.set(false);}
                    >
                        Cancel
                    </button>
                </div>
            </div>
        </div>
    }
}
