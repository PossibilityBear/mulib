use crate::components::song::song::Song;
use crate::components::song_list::song_list::{
    DelFromPlaylistOpt, DelFromQueueOpt, SongActionOpts, SongList,
};
use crate::{
    components::controls::controls::PlaybackState,
    models::{
        // album::{Album, AlbumDBModel},
        // artist::{Artist, ArtistDBModel},
        song::Song,
    },
};
use leptos::prelude::*;
use std::collections::VecDeque;
use stylance::import_crate_style;
use uuid::Uuid;

type SongQueue = RwSignal<VecDeque<QueueEntry>>;

#[derive(Default, Clone, Copy)]
struct ListQueue {
    songs: SongQueue,
    shuffled: RwSignal<bool>,
}

#[derive(Copy, Clone, Debug)]
pub enum QueueType {
    /// the manual queue with songs added by the user
    User,
    /// the queue from the currently playing list of songs
    /// with a boolean to determine if its shuffled or not
    List,
}

#[derive(Default, Clone, Copy)]
pub struct SongQueueContext {
    user_queue: SongQueue,
    list_songs: ListQueue,
    /// the requested operation on the queue, play, pause, skip, etc...
    playback_state: RwSignal<PlaybackState>,
}

#[derive(Debug, Clone)]
pub struct QueueEntry {
    pub song: Song,
    pub id: Uuid,
}
impl Into<QueueEntry> for Song {
    fn into(self) -> QueueEntry {
        QueueEntry {
            song: self,
            id: Uuid::new_v4(),
        }
    }
}
impl Into<Song> for QueueEntry {
    fn into(self) -> Song {
        self.song
    }
}

impl SongQueueContext {
    /// toggles if playing lists of songs will
    /// be shuffled or not.
    pub fn toggle_shuffle(&self) {
        self.list_songs
            .shuffled
            .set(!self.list_songs.shuffled.get())
        // TODO: once its clear how shuffle works, this should
        // shuffle / unshuffle the queue as well.
    }

    /// Add a song to the front of the user queue
    pub fn push_front(&self, song: Song) {
        self.user_queue.update(|sq| {
            let entry = song.clone().into();
            sq.push_front(entry);
        });
    }

    /// Add a set of songs to the back of the user queue
    pub fn push_back(&self, songs: Vec<Song>) {
        self.user_queue.update(|sq| {
            let mut songs: VecDeque<QueueEntry> = songs.into_iter().map(|s| s.into()).collect();
            sq.append(&mut songs);
        });
    }

    /// set the list queue to the provided songs
    pub fn set_list_queue(&self, songs: Vec<Song>) {
        self.list_songs
            .songs
            .set(VecDeque::from_iter(songs.into_iter().map(|s| s.into())));
    }

    /// helper function to remove items from a vec based on a provided set of
    /// indicies
    fn remove_tracks(songs: &mut VecDeque<QueueEntry>, mut track_nums: Vec<usize>) {
        // Given that the track order is sorted in the song list, we can then
        // sort the track remove list and iterate just once over song list
        // removing tracks in order O(n log(n)) + O(n)) vs O(n^2) from nested
        // by element comparison
        track_nums.sort();

        let t_start_len: usize = track_nums.len();
        // copy songs so we have stable indicies to reference
        for (i, _) in songs.clone().into_iter().enumerate() {
            if let Some(remove_t) = track_nums.first() {
                if &i == remove_t {
                    // remove the track, adjusting index based on how many
                    // tracks have already been removed.
                    songs.remove(remove_t - (t_start_len - track_nums.len()));
                    // update list of tracks to remove
                    track_nums.remove(0);
                }
            } else {
                // no more tracks to remove break early
                break;
            }
        }
    }

    /// remove songs from the provided queue type
    pub fn remove_songs(&self, queue_type: QueueType, track_nums: Vec<usize>) {
        match queue_type {
            QueueType::User => {
                self.user_queue
                    .update(|sq| Self::remove_tracks(sq, track_nums));
            }
            QueueType::List => {
                self.list_songs
                    .songs
                    .update(|sq| Self::remove_tracks(sq, track_nums));
            }
        }
    }

    /// Pops the next song off of the queue, pulls from user queue
    /// and then list queue if the user queue is empty
    pub fn pop_front(&self) -> Option<QueueEntry> {
        let mut song = Option::<QueueEntry>::None;
        self.user_queue.update(|sq| {
            song = sq.pop_front();
        });

        if song.is_none() {
            self.list_songs.songs.update(|sq| {
                song = sq.pop_front();
            });
        }

        return song;
    }

    /// reads the next song off the queue, reads from user queue
    /// and then list queue if ht user queue is empty
    pub fn peek_front(&self) -> Option<QueueEntry> {
        let mut song: Option<QueueEntry>;

        song = self.user_queue.get().front().map(|s| s.clone());

        if song.is_none() {
            song = self.list_songs.songs.get().front().map(|s| s.clone());
        }

        song
    }

    /// gets the next song from the specified queue type
    pub fn get_songs(&self, queue_type: QueueType) -> Memo<Vec<Song>> {
        match queue_type {
            QueueType::User => {
                let songs_sig = self.user_queue;
                Memo::new(move |_| songs_sig.get().into_iter().map(|q| q.into()).collect())
            }
            QueueType::List => {
                let songs_sig = self.list_songs.songs;
                Memo::new(move |_| songs_sig.get().into_iter().map(|q| q.into()).collect())
            }
        }
    }

    pub fn get_playback_state(&self) -> PlaybackState {
        self.playback_state.get()
    }

    pub fn set_playback_state(&self, state: PlaybackState) {
        *self.playback_state.write() = state;
    }
}

import_crate_style!(queue, "./src/components/queue/queue.module.scss");
#[component]
pub fn Queue() -> impl IntoView {
    let song_queue: SongQueueContext = use_context::<SongQueueContext>()
        .expect("to have found song queue context")
        .into();

    let actions = SongActionOpts {
        add_to_queue: true,
        add_to_playlist: true,
        remove_from_playlist: DelFromPlaylistOpt::False,
        remove_from_queue: DelFromQueueOpt::True(QueueType::User),
    };

    view! {
        <div class=queue::container>
            <h1 class=queue::Title> "Queue" </h1>
            <div class=queue::songs>
                <SongList songs=song_queue.get_songs(QueueType::User) actions/>

                // TODO: add a second list for the list_songs
                <h2 class=queue::Header>Next Up</h2>
                <SongList songs=song_queue.get_songs(QueueType::List) actions/>
            </div>
        </div>
    }
}
