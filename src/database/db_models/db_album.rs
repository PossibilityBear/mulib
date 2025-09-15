use sqlx::prelude::FromRow;

use crate::models::{album::Album, artist::Artist};

#[derive(Debug, Clone, FromRow)]
pub struct DbAlbum {
    pub id: i64,
    pub title: String,
    pub artist_id: Option<i64>,
    pub artist_name: Option<String>,
}


impl Into<Album> for DbAlbum {
    fn into(self) -> Album {
        let artist = match (self.artist_id, self.artist_name) {
            (Some(id), Some(name)) => Some(Artist {id, name}),
            (Some(id), None) => Some(Artist {id, name: String::new()}),
            (_, _) => None
        };

        Album { 
            id: self.id, 
            title: self.title, 
            artist: artist
        }
    }
}