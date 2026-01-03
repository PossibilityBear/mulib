use crate::components::queue::queue::SongQueueContext;
use leptos::{
    ev,
    html::{self},
    leptos_dom::logging::console_log,
    prelude::*,
};
use leptos_svg::svg;
use leptos_use::use_event_listener;
use stylance::import_crate_style;

import_crate_style!(main_style, "./src/styles/main.module.scss");
import_crate_style!(controls, "./src/components/controls/controls.module.scss");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaybackState {
    Play,
    Pause,
    SkipForward,
    SkipBackward,
}
impl Default for PlaybackState {
    fn default() -> PlaybackState {
        PlaybackState::Pause
    }
}

#[derive(Default, Clone, Copy)]
struct SongProgress {
    duration: f64,
    current: f64,
}

#[component]
pub fn Controls(queue: SongQueueContext, show_queue: RwSignal<bool>) -> impl IntoView {
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
            }
            PlaybackState::Pause => {
                if let Some(audio_el) = audio_ref.get() {
                    _ = audio_el.pause();
                }
            }
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
            }
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

    // potentially todo:
    // Create a function like proc macro that
    // 1. parses svg file
    // 2. strips xml info, comments, inline styles, etc
    // 3. takes a vec of classes Strings to apply to svg
    // 4. applies classes to svg
    // 5. processed svg into a view!{} macro for use in leptos

    // this proc macro would save a bunch of manual work,

    let play_svg = svg!(
        "./public/play.svg",
        main_style::svg_button,
        controls::play_svg
    );

    let pause_svg = svg!("./public/pause.svg", main_style::svg_button);

    let seek_forward_svg = svg!("./public/seek-forward.svg", main_style::svg_button);

    let volume_svg = svg!("./public/volume-icon.svg", main_style::svg_button);

    let hide_queue_svg = svg!("./public/hide-queue.svg", main_style::svg_button);

    let show_queue_svg = svg!("./public/show-queue.svg", main_style::svg_button);

    // TODO: replace input type = image with buttons wrapped around svgs
    view! {
        <div>
            <audio
                node_ref=audio_ref
                on:timeupdate=on_time_update
                autoplay=move || {queue.get_playback_state() == PlaybackState::Play}
                src = move || {
                    match queue.peek_front() {
                        Some(entry) => Some(entry.song.file_path.clone()),
                        None => None
                    }
                }
            >
            </audio>
            <div class=controls::input_group>
                // playback controls
                <button class=main_style::svg_button
                    // Pause / Play
                    on:click=move |_| {
                        if queue.get_playback_state() == PlaybackState::Pause {
                            queue.set_playback_state(PlaybackState::Play);
                        } else {
                            queue.set_playback_state(PlaybackState::Pause);
                        }
                    }
                >
                    <Show
                        when=move|| {queue.get_playback_state() == PlaybackState::Pause}
                        fallback=move || {pause_svg}
                    >
                        {play_svg}
                    </Show>
                </button>

                <button class=main_style::svg_button
                    // skip forward
                    on:click=move |_| {
                        queue.set_playback_state(PlaybackState::SkipForward);
                    }
                > { seek_forward_svg } </button>

                // volume controls
                <div class=controls::input_group>
                    {volume_svg}
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
                <button class=main_style::svg_button
                    on:click=toggle_queue
                >
                    <Show
                        when= move || {show_queue.get()}
                        fallback=show_queue_svg
                    >
                        {hide_queue_svg}
                    </Show>
                </button>
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
                    if queue.peek_front().is_some()  {
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
