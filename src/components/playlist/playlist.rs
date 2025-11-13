use leptos::prelude::*;
use leptos_svg::svg;
use stylance::import_crate_style;
use crate::{components::song_list::song_list::SongListSource, models::playlist::Playlist};

import_crate_style!(playlist, "./src/components/playlist/playlist.module.scss");

#[component]
pub fn PlaylistCard(list: Playlist) -> impl IntoView {
    // let song_count = list.songs.len();
    let list_source = use_context::<RwSignal<SongListSource>>().expect("To have found song list source context");

    let (list, _) = signal(list);
    view!{
        <div class=move || {
            if let SongListSource::Playlist(p) = list_source.get() {
                if p.id == list.get().id {
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
                >{ move || list.get().title}</p>
                // <p class=playlist::SongCount>{song_count} songs</p>
            </div>
        </div>
    }
}