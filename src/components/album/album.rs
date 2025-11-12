use leptos::prelude::*;
use stylance::import_crate_style;
use crate::{components::song_list::song_list::SongListSource, models::album::Album};

import_crate_style!(album, "./src/components/album/album.module.scss");

#[component]
pub fn AlbumCard(album: Album) -> impl IntoView {
    // let song_count = list.songs.len();
    let list_source = use_context::<RwSignal<SongListSource>>().expect("To have found song list source context");

    let alb_id = album.id;
    let alb_cp = album.clone();
    view!{
        <div class=move || {
            if let SongListSource::Album(a) = list_source.get()  {
                if (a.id == alb_id) {
                    return vec![album::AlbumCard, album::Selected].join(" ")
                }
            } 
            album::AlbumCard.to_string()
        }>
            <div class=album::TextColGroup>
                <p class=album::Name
                    on:click=move |_| {
                        list_source.set(SongListSource::Album(alb_cp.clone()))
                    }
                >{ album.title }</p>
                // <p class=playlist::SongCount>{song_count} songs</p>
            </div>
        </div>
    }
}