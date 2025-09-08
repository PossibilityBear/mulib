use std::collections::VecDeque;
use uuid::Uuid;
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
    songs: RwSignal<VecDeque<QueueEntry>>,
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

#[derive(Debug, Clone)]
pub struct QueueEntry {
    pub song: Song,
    pub id: Uuid
}
impl Into<QueueEntry> for Song {
    fn into(self) -> QueueEntry {
        QueueEntry { song: self, id: Uuid::new_v4()}
    }
}
impl Into<Song> for QueueEntry {
    fn into(self) -> Song {
        self.song
    }
}


impl SongQueue {
    pub fn push_front(&self, song: Song) {
        self.context.songs.update(|sq| {
            let entry = song.clone().into();
            sq.push_front(entry);
        });
    }

    pub fn push_back(&self, song: Song) {
        self.context.songs.update(|sq| {
            let entry = song.clone().into();
            sq.push_back(entry);
        });
    }

    pub fn add_songs(&self, songs: Vec<Song>) {
        self.context.songs.update(|sq| {
            let songs = songs.clone();
            let mut vdq: VecDeque::<QueueEntry> = songs.iter()
                .map(|song| {(*song).clone().into()})
                .collect();
            sq.append(&mut vdq);
        });
    } 

    pub fn remove_songs(&self, entry_id: Uuid) {
        self.context.songs.update(|sq| {
            if let Some(song_index) = sq.iter().position(|x| x.id == entry_id) {
                sq.remove(song_index);
            }
        });
    }

    pub fn pop_front(&self) -> Option<QueueEntry> {
        let mut song = Option::<QueueEntry>::None;
        self.context.songs.update( |sq| {
            song = sq.pop_front();
        });
        return song;
    }

    pub fn peek_front(&self) -> Option<QueueEntry> {
        match self.context.songs.get().front() {
            Some(s) => {
                return Some((*s).clone())
            },
            None => return None
        }
    }

    pub fn get_songs(&self) -> VecDeque<QueueEntry> {
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
                                song=Some(song.into())
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