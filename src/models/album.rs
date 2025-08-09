use crate::models::artist::Artist;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<u32>,
    pub title: String,
    pub artist: Option<Artist>
}
#[derive(Debug, Clone)]
pub struct AlbumDBModel {
    pub id: u32,
    pub title: String,
    pub artist_id: Option<u32>
}

