use std::{clone, collections::VecDeque, default, time::Duration};
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

#[derive (Default, Clone, Copy)]
struct SongProgress {
    duration: f64 ,
    current: f64 ,
}

#[component]
pub fn Controls(
    queue: SongQueue
) -> impl IntoView {

    let audio_ref = NodeRef::<html::Audio>::new();
    let song_progress_ref = NodeRef::<html::Input>::new();
    let volume_ref = NodeRef::<html::Input>::new();

    _ = use_event_listener(audio_ref, ev::ended, move |_| {
        console_log("Audio track has finished playing!");
        queue.pop_front();
    });

    let (song_progress, set_song_progress) = signal(SongProgress::default());

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

                // if let Some(audio_el) = audio_ref.get() {
                //     // technically autoplay is controlling
                //     // this but doesn't seem to hurt
                //     _ = audio_el.play();
                // }

            },
            PlaybackState::SkipBackward => todo!(),
        }
    });
    let on_time_update = move |_| {
        set_song_progress.update(|sp| {
            if let Some(audio_el) = audio_ref.get() {
                sp.duration = audio_el.duration();
                sp.current = audio_el.current_time();
            } else {
                sp.current = 0.0;
            }
        })
    };

    


    view!{
        <div>
            <For
                each=move || queue.get_songs()
                key=|song| song.id 
                children=move |song| {
                    view!{
                        <p class=controls::now_playing
                        on:click=move |_| {
                            queue.remove_songs(song.id.expect("song to have ID"));
                        }
                        >{song.title}</p>
                    }
                }
            />
            <audio 
            node_ref=audio_ref 
            class=main_style::centered
            on:timeupdate=on_time_update
            autoplay=move || {queue.get_playback_state() == PlaybackState::Play} 
            src = move || {
                match queue.peek_front() {
                    Some(song) => Some(song.file_path.clone()),
                    None => None
                }
            }>
            </audio>
            <div class=controls::input_group>
                <input
                    type="image"
                    src=move || {
                        if queue.get_playback_state() == PlaybackState::Pause {
                            "/public/play.svg"
                        } else {
                            "/public/pause.svg"
                        }
                    }
                    class=controls::button
                    on:click=move |_| {
                        if queue.get_playback_state() == PlaybackState::Pause {
                            queue.set_playback_state(PlaybackState::Play);
                        } else {
                            queue.set_playback_state(PlaybackState::Pause);
                        }
                    }
                /> 
                <input
                    type="image"
                    src="/public/seek-forward.svg"
                    class=controls::button
                    on:click=move |_| {
                        queue.set_playback_state(PlaybackState::SkipForward);
                    }
                /> 
                <div class=controls::input_group>
                    <image class=controls::button src="/public/volume-icon.svg"/>
                    <input type="range" 
                        node_ref=volume_ref
                        min="0.0" 
                        max="1.0"
                        step="0.01"
                        prop:value="1.0"
                        on:change=move |_| {
                            if let (Some(range), Some(audio)) = (volume_ref.get(), audio_ref.get()) {
                                console_log(&range.value());
                                audio.set_volume(range.value().parse::<f64>().expect("to convert range value to float"));
                            }
                        }
                    />
                </div>
            </div>
            <div class=controls::input_group>
                <input type="range" 
                    min="0" 
                    node_ref=song_progress_ref
                    max=move || {song_progress.get().duration} 
                    prop:value=move || {
                        if queue.peek_front().is_none() {
                            0.0
                        } else {
                            song_progress.get().current
                        }
                    }
                    on:change=move |event| {
                        if let (Some(range), Some(audio)) = (song_progress_ref.get(), audio_ref.get()) {
                            audio.set_current_time(range.value().parse::<f64>().expect("to convert range value to float"));
                        }
                        // todo!()
                    }
                />
                <p class=controls::time_stamp> {move || {
                    if queue.peek_front().is_some() && queue.get_playback_state() == PlaybackState::Play {
                        let mut duration = song_progress.get().duration;
                        if duration.is_nan() {
                            duration = 0.0;
                        }
                        let current = song_progress.get().current;

                        let current_minutes = (current / 60.0).floor();
                        let current_seconds = current % 60.0;

                        let duration_minutes = (duration / 60.0).floor();
                        let duration_seconds = duration % 60.0;

                        format!("{current_minutes:01.0}:{current_seconds:02.0} / {duration_minutes:01.0}:{duration_seconds:02.0}")
                    } else {
                        format!("0:00 / 0:00")
                    }
                }}</p>
            </div>
        </div>
    }
}