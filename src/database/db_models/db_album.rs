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
        let artist = Artist::opt_new(self.artist_id, self.artist_name);

        Album { 
            id: self.id, 
            title: self.title, 
            artist: artist
        }
    }
}