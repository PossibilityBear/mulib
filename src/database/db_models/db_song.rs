use sqlx::prelude::FromRow;

use crate::models::{album::Album, artist::Artist, song::Song};

#[derive(Debug, Clone, FromRow)]
pub struct DbSong {
    pub id: i64,
    pub title: String,
    pub file_path: String,
    pub artist_id: Option<i64>,
    pub artist_name: Option<String>,
    pub album_id: Option<i64>, 
    pub album_title: Option<String>,
    pub album_artist_id: Option<i64>,
    pub album_artist_name: Option<String>,
}


impl Into<Song> for DbSong {
    fn into(self) -> Song {
        let artist = match (self.artist_id, self.artist_name) {
            (Some(id), Some(name)) => Some(Artist {id, name}),
            (Some(id), None) => Some(Artist {id, name: String::new()}),
            (_, _) => None
        };

        let alb_artist = match (self.album_artist_id, self.album_artist_name) {
            (Some(id), Some(name)) => Some(Artist {id, name}),
            (Some(id), None) => Some(Artist {id, name: String::new()}),
            (_, _) => None
        };

        let album = match (self.album_id, self.album_title) {
            (Some(id), Some(title)) => Some(Album {id, title, artist: alb_artist}),
            (Some(id), None) => Some(Album {id, title: String::new(), artist: alb_artist}),
            (_, _) => None,
        };

        Song {
            id: self.id,
            title: self.title,
            file_path: self.file_path,
            artist: artist,
            album: album,
        }
    }
}