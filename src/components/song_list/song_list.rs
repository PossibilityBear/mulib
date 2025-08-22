use std::{clone, collections::VecDeque};
use leptos::prelude::*;
use leptos_struct_table::*;
use std::ops::Range;
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::models::{
        album::{Album, AlbumDBModel}, 
        artist::{Artist, ArtistDBModel}, 
        song::Song
    };
    


#[derive(Serialize, Deserialize, Clone)]
pub struct SongData {
    title: String,
    author: String,
    album: String,
    song_id: u32,
}
impl Default for SongData {
    fn default() -> Self {
        Self { 
            title: "loading....".to_string(), 
            author: "loading....".to_string(), 
            album:  "loading....".to_string(), 
            song_id: 0,
        }
    }
}

#[server]
pub async fn get_songs(list_id: u32) -> Result<Vec<Song>, ServerFnError> {
    use crate::database::commands::get_songs::get_songs;
    use crate::database::utils::db_connection::*;

    println!("hello from get songs");
    let conn = DbConnection::default(); 
    let songs = get_songs(conn);

    Ok(songs)

}

#[server]
pub async fn get_song_count(list_id: u32) -> Result<usize, ServerFnError> {
    use crate::database::utils::db_connection::*;

    println!("hello from get songs");
    let conn = DbConnection::default(); 

    let db = conn.db();
    let mut stmt = db.prepare(
        "
        SELECT 
            Count(*) AS SongCount
        FROM Song AS s
        "
    ).unwrap();

    let song_counts = stmt.query_map([], |row| {
        let count: usize = row.get(0)?;
        Ok(count)
    }).unwrap();

    let count: usize = *song_counts
        .map(|count| count.expect("to have gotten count"))
        .collect::<Vec<usize>>()
        .first().unwrap_or(&0); 

    Ok(count)

}


import_crate_style!(main_style, "./src/styles/main.module.scss");
// a single song
#[component] 
pub fn Song(song: Song) -> impl IntoView {
    let now_playing = use_context::<WriteSignal<Option<Song>>>().expect("to have found now playing song");
    let song_copy = song.clone();
    view! {
        <p>
            {format!("Title: {}", song.title)}
        </p>
        <p>
            {format!("Author: {}", song.artist.unwrap_or_default().name)}
        </p>
        <p>
            {format!("Album: {}", song.album.unwrap_or_default().title)}
        </p>
        <button on:click= move |_|{
            *now_playing.write() = Some(song_copy.clone());
        }>{"play"}</button>
    }
}
#[derive(Default)]
pub struct SongListDataProvider {
    list_id: RwSignal<u32>,
    sort: VecDeque<(usize, ColumnSort)>,
}

impl TableDataProvider<Song> for SongListDataProvider {
    async fn get_rows(
        &self, range: Range<usize>
    ) -> Result<(Vec<Song>, Range<usize>), String> {
        println!("Hellof from data provider");

        // was getting weird errors from 
        // leptos table in browser console, fix 
        // was to change range returned to be
        // based on range.start..range.start + len of vec
        // then i changed it to range.end to fix duplication
        // issues where all elements were duped.
        match get_songs(self.list_id.get_untracked()).await {
            Ok(songs) => {
                let len = songs.len();
                Ok((songs, range.start..range.end))
            },
            Err(msg) => Err(format!("{:?}", msg)),
        }
        
    }

    async fn row_count(&self) -> Option<usize> {
        get_song_count(self.list_id.get_untracked()).await.ok()
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sort = sorting.clone();
    }

    fn track(&self) {
        self.list_id.track();
    }
}

#[derive(TableRow, Debug, Clone, Serialize, Deserialize)]
#[table(impl_vec_data_provider)]
pub struct Num {
    val: i32
}
pub struct NumDataProvider {}

impl TableDataProvider<Num> for NumDataProvider {
    async fn get_rows(
        &self, range: Range<usize>
    ) -> Result<(Vec<Num>, Range<usize>), String> {
        let nums = vec![
            Num {val: 1},
            Num {val: 2},
            Num {val: 3},
            Num {val: 4},

        ];
        let nums_len: usize = nums.len();
        Ok((nums, range.start..nums_len))
    }
}

// a list of songs from database
import_crate_style!(style, "./src/components/song_list/song_list.module.scss");
#[component]
pub fn SongList (
    list_id: u32
) -> impl IntoView {
    let now_playing = use_context::<WriteSignal<Option<Song>>>().expect("to have found now playing song");


    let selected_index = RwSignal::new(None);
    let (selected_row, set_selected_row) = signal(Option::<Signal<Song>>::None);
    // let (selected_row, set_selected_row) = signal(Option::<Signal<Num>>::None);

    view! {
        <div>
            <table class=style::songs> 
                <TableContent 
                    selection=Selection::Single(selected_index)
                    on_selection_change={move |evt: SelectionChangeEvent<Song>| {
                        set_selected_row.write().replace(evt.row);
                        let song = evt.row.get().clone();
                        *now_playing.write() = Some(song);
                    }}
                    row_class="select-none"
                    rows={SongListDataProvider::default()} 
                    sorting_mode=SortingMode::SingleColumn
                    scroll_container="html"/>
            </table>
            <p class=style::now_playing> {move || {
                match selected_row.get() {
                    Some(sig) => format!("Now Playing: {}", sig.get().title),
                    None => format!("Now Playing: <None>")
                }}
            } </p>
        </div>
    }

}