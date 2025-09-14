use leptos::{ev, html::{self}, leptos_dom::logging::console_log, prelude::*};
use leptos_use::{use_event_listener};
use stylance::import_crate_style;
use crate::components::queue::queue::SongQueue;
    
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

#[derive (Default, Clone, Copy)]
struct SongProgress {
    duration: f64 ,
    current: f64 ,
}

#[component]
pub fn Controls(
    queue: SongQueue,
    show_queue: RwSignal<bool>
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

    
    let toggle_queue = move |_| {
        *show_queue.write() = !show_queue.get();
    };


    view!{
        <div>
            <audio 
            node_ref=audio_ref 
            class=main_style::centered
            on:timeupdate=on_time_update
            autoplay=move || {queue.get_playback_state() == PlaybackState::Play} 
            src = move || {
                match queue.peek_front() {
                    Some(entry) => Some(entry.song.file_path.clone()),
                    None => None
                }
            }>
            </audio>
            <div class=controls::input_group>
                // playback controls
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
                // volume controls
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
                // Queue visibility toggle
                <input 
                    type="image" 
                    src={move || {if show_queue.get() {"/public/hide-queue.svg"} else {"/public/show-queue.svg"}}}
                    on:click=toggle_queue
                />
                </div>
                // Song Progress Bar
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
                        on:change=move |_event| {
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
            <div class=controls::input_group>
            </div>
        </div>
    }
}