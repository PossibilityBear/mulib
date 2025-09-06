use std::{clone, collections::VecDeque, default};
use leptos::{ev, html::{self, p}, leptos_dom::logging::console_log, prelude::*, tachys::html::style};
use leptos_use::{use_event_listener, use_window};
use stylance::import_crate_style;
use serde::{Serialize, Deserialize};
use crate::models::{
        album::{Album, AlbumDBModel}, 
        artist::{Artist, ArtistDBModel}, 
        song::Song
    };
    
import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(controls, "./src/components/controls/controls.module.scss");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaybackState {
   Play,
   Pause,
   SkipForward,
   SkipBackward
}
impl Default for PlaybackState {
    fn default() -> PlaybackState {
        PlaybackState::Pause
    }
}

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



#[component]
pub fn Controls(
    queue: SongQueue
) -> impl IntoView {

    let audio_ref = NodeRef::<html::Audio>::new();

    _ = use_event_listener(audio_ref, ev::ended, move |_| {
        console_log("Audio track has finished playing!");
        queue.pop_front();
    });


    Effect::new(move |_| {
        match queue.get_playback_state() {
            PlaybackState::Play => {
                if queue.peek_front().is_none() {
                    return;
                }
                if let Some(audio_el) = audio_ref.get() {
                    _ = audio_el.play();
                }
            },
            PlaybackState::Pause => {
                if let Some(audio_el) = audio_ref.get() {
                    _ = audio_el.pause();
                }
            },
            PlaybackState::SkipForward => {
                if queue.peek_front().is_none() {
                    queue.set_playback_state(PlaybackState::Pause);
                    return;
                }

                _ = queue.pop_front();

                if queue.peek_front().is_none() {
                    queue.set_playback_state(PlaybackState::Pause);
                    return;
                }

                queue.set_playback_state(PlaybackState::Play);

                if let Some(audio_el) = audio_ref.get() {
                    // technically autoplay is controlling
                    // this but doesn't seem to hurt
                    _ = audio_el.play();
                }

            },
            PlaybackState::SkipBackward => todo!(),
        }
    });

    view!{
        <div>
            <p class=controls::now_playing> {move || {
                    format!("Play state = {:?} ", queue.get_playback_state())
                }}
            </p>
            <p class=controls::now_playing> {move || {
                match queue.peek_front() {
                    Some(song) => format!("Now Playing: {}", song.title),
                    None => format!("Now Playing: <None>")
                }}
            }
            </p>
            <ol>
                <For
                    each=move || queue.get_songs()
                    key=|song| song.id 
                    children=move |song| {
                        view!{
                            <p 
                            on:click=move |_| {
                                queue.remove_songs(song.id.expect("song to have ID"));
                            }
                            >{song.title}</p>
                        }
                    }
                />
            </ol>
            <audio 
            node_ref=audio_ref 
            class=main_style::centered
            controls 
            autoplay=move || {queue.get_playback_state() == PlaybackState::Play} 
            src = move || {
                match queue.peek_front() {
                    Some(song) => Some(song.file_path.clone()),
                    None => None
                }
            }>
            </audio>
            <button
                on:click=move |_| {
                    queue.set_playback_state(PlaybackState::Play);
                }
            > 
                "play" 
            </button>
            <button
                on:click=move |_| {
                    queue.set_playback_state(PlaybackState::Pause);
                }
            > 
                "pause" 
            </button>
            <button
                on:click=move |_| {
                    queue.set_playback_state(PlaybackState::SkipForward);
                }
            > 
                "skip" 
            </button>
        </div>
    }
}