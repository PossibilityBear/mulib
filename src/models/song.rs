use serde::{Deserialize, Serialize};
use crate::models::{album::Album, artist::Artist};
use leptos::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: Option<u32>,
    pub title: String,
    pub file_path: String,
    pub artist: Option<Artist>,
    pub album: Option<Album>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongDBModel {
    pub id: u32,
    pub title: String,
    pub file_path: String,
    pub artist_id: Option<u32>,
    pub album_id: Option<u32>, 
}