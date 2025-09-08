use std::collections::VecDeque;
use leptos::prelude::*;
use stylance::import_crate_style;
use crate::components::song::song::Song;
use crate::{components::controls::controls::PlaybackState, models::{
        // album::{Album, AlbumDBModel}, 
        // artist::{Artist, ArtistDBModel}, 
        song::Song
    }};

    

#[derive(Default, Clone, Copy)]
pub struct SongQueueContext {
    songs: RwSignal<VecDeque<Song>>,
    playback_state: RwSignal<PlaybackState>
}

#[derive(Default, Clone, Copy)]
pub struct SongQueue {
    context: SongQueueContext,
}

impl Into::<SongQueue> for SongQueueContext {
    fn into(self) -> SongQueue {
        SongQueue{context: self}
    }
}


impl SongQueue {
    pub fn push_front(&self, song: Song) {
        self.context.songs.update(|sq| {
            let song = song.clone();
            sq.push_front(song);
        });
    }

    pub fn push_back(&self, song: Song) {
        self.context.songs.update(|sq| {
            let song = song.clone();
            sq.push_back(song);
        });
    }

    pub fn add_songs(&self, songs: Vec<Song>) {
        self.context.songs.update(|sq| {
            let songs = songs.clone();
            let mut vdq_songs: VecDeque::<Song> = songs.into();
            sq.append(&mut vdq_songs);
        });
    } 

    pub fn remove_songs(&self, song_id: u32) {
        self.context.songs.update(|sq| {
            if let Some(song_index) = sq.iter().position(|x| x.id == Some(song_id)) {
                sq.remove(song_index);
            }
        });
    }

    pub fn pop_front(&self) -> Option<Song> {
        let mut song = Option::<Song>::None;
        self.context.songs.update( |sq| {
            song = sq.pop_front();
        });
        return song;
    }

    pub fn peek_front(&self) -> Option<Song> {
        match self.context.songs.get().front() {
            Some(s) => {
                return Some((*s).clone())
            },
            None => return None
        }
    }

    pub fn get_songs(&self) -> VecDeque<Song> {
        self.context.songs.get()
    }

    pub fn get_playback_state(&self) -> PlaybackState {
        self.context.playback_state.get()
    }

    pub fn set_playback_state(&self, state:  PlaybackState) {
        *self.context.playback_state.write() = state;
    }
}


import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(queue, "./src/components/queue/queue.module.scss");
#[component]
pub fn Queue() -> impl IntoView {
    let song_queue: SongQueue = use_context::<SongQueueContext>().expect("to have found song queue context").into();
    view!{
        <div class=queue::container>
            <h1> "Queue" </h1>
            <div class=queue::songs>
                <For
                    each=move || song_queue.get_songs()
                    key=|song| song.id 
                    children=move |song| {
                        view!{
                            <Song
                                song=Some(song)
                                actions={vec![]}
                            // on:click=move |_| {
                            //     queue.remove_songs(song.id.expect("song to have ID"));
                            // }
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}