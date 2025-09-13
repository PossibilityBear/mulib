use serde::{Deserialize, Serialize};
use crate::models::{album::{Album, ParsedAlbum}, artist::{Artist, ParsedArtist}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Song {
    pub id: i64,
    pub title: String,
    pub file_path: String,
    pub artist: Option<Artist>,
    pub album: Option<Album>, 
}


#[derive(Debug, Clone, PartialEq)]
pub struct ParsedSong {
    pub id: Option<i64>,
    pub title: String,
    pub file_path: String,
    pub artist: Option<ParsedArtist>,
    pub album: Option<ParsedAlbum>, 
}

impl Into<ParsedSong> for Song {
    fn into(self) -> ParsedSong {
        ParsedSong { 
            id: Some(self.id), 
            title: self.title, 
            file_path: self.file_path, 
            artist: if let Some(a) = self.artist {
                Some(a.into())
            } else {
                None
            }, 
            album: if let Some(a) = self.album {
                Some(a.into())
            } else {
                None
            }
        }
    }
}