use crate::models::artist::{Artist, ParsedArtist};

use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub artist: Option<Artist>
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ParsedAlbum {
    pub id: Option<i64>,
    pub title: String,
    pub artist: Option<ParsedArtist>
}

impl Into<ParsedAlbum> for Album {
    fn into(self) -> ParsedAlbum {
        ParsedAlbum{
            id: Some(self.id),
            title: self.title,
            artist: if let Some(a) = self.artist {
                Some(a.into())
            } else {
                None
            }
        }
    }
}




