use leptos::prelude::*;
use stylance::import_crate_style;
use crate::components::song_list::song_list::{SongList, SongListSource};

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

#[component]
pub fn LibrarySidebar() -> impl IntoView {
    let tab_selection = RwSignal::new(Tabs::Playlists);
    let (list_source, _set_list_source) = signal(SongListSource::All); 

    view!{
        <div class=library::LibContainer>
            <div class=library::HeaderRow>
                <h1 class=library::Title> Library </h1>
            </div>
            <div class=library::HeaderRow>
                <TabSelector tab=Tabs::Artists tab_selection=tab_selection/>
                <TabSelector tab=Tabs::Albums tab_selection=tab_selection/>
                <TabSelector tab=Tabs::Playlists tab_selection=tab_selection/>
            </div>
            <SongList source=list_source/>
        </div>
    }
}