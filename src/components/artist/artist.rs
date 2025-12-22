use crate::{components::song_list::song_list::SongListSource, models::artist::Artist};
use leptos::prelude::*;
use stylance::import_crate_style;

import_crate_style!(artist, "./src/components/artist/artist.module.scss");

#[component]
pub fn ArtistCard(artist: Artist) -> impl IntoView {
    // let song_count = list.songs.len();
    let list_source =
        use_context::<RwSignal<SongListSource>>().expect("To have found song list source context");

    let (artist, _) = signal(artist);
    view! {
        <div class=move || {
            if let SongListSource::Artist(a) = list_source.get()  {
                if a.id == artist.get().id {
                    return vec![artist::ArtistCard, artist::Selected].join(" ")
                }
            }
            artist::ArtistCard.to_string()
        }
        on:click=move |_| {
            list_source.set(SongListSource::Artist(artist.get()))
        }
        >
            <div class=artist::TextColGroup>
                <p class=artist::Name > { move || artist.get().name}</p>
            </div>
        </div>
    }
}
