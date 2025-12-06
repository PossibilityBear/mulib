use crate::components::controls::controls::Controls;
use crate::components::library::library::LibrarySidebar;
use crate::components::queue::queue::{Queue, SongQueueContext};
use crate::components::song_list::song_list::{SongList, SongListSource};
use crate::models::playlist::{Playlist, PlaylistsSource2, PlaylistsSource2StoreFields};
use leptos::prelude::*;
use reactive_stores::Store;
use stylance::import_crate_style;

import_crate_style!(
    home_page,
    "./src/components/home_page/home_page.module.scss"
);

#[server(prefix = "/api", endpoint = "get_playlists")]
pub async fn get_playlists() -> Result<Vec<Playlist>, ServerFnError> {
    use crate::app_state::AppState;
    use crate::database::commands::playlists::get_playlists_info;

    let state = use_context::<AppState>().expect("To have Found App State");

    let playlists = get_playlists_info(&state.db).await?;

    Ok(playlists)
}

#[component]
pub fn HomePage() -> impl IntoView {
    let show_queue = RwSignal::<bool>::new(false);

    let queue_context = SongQueueContext::default();
    provide_context(queue_context);

    let list_source = RwSignal::new(SongListSource::All);
    provide_context(list_source);

    // define playlists context
    let playlists = Store::new(PlaylistsSource2::new());
    provide_context(playlists);

    // define action to load playlists from database
    let load_playlists_action = Action::new(move |_: &()| async move {
        if !playlists.is_loaded().get_untracked() {
            if let Ok(lists) = get_playlists().await {
                leptos::logging::log!("got playlists");
                playlists.lists().set(lists);
                playlists.is_loaded().set(true);
            }
        }
    });

    // Use action to load the playlist on component mount
    Effect::new(move |_| {
        leptos::logging::log!("loading playlists");
        load_playlists_action.dispatch(());
    });

    view! {
        <div class=home_page::container>
            <div class=home_page::main_view>
                <div class=home_page::Library>
                    <LibrarySidebar/>
                </div>
                <div class=home_page::song_list>
                    <SongList source=list_source/>
                </div>
                {move || {
                    if show_queue.get() {
                        Some(view!{<div class=home_page::queue><Queue/></div>})
                    } else {
                        None
                    }
                }}
            </div>
            <div class=home_page::controls>
                <Controls queue=queue_context.into() show_queue=show_queue />
            </div>
        </div>

    }
}
