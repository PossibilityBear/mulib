
use serde::{Deserialize, Serialize};

use crate::models::song::Song;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Playlist {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub songs: Vec<Song>
}