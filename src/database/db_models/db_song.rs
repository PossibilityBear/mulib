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
        let artist = Artist::opt_new(self.artist_id, self.artist_name);

        let alb_artist = Artist::opt_new(self.album_artist_id, self.album_artist_name);
        let album = Album::opt_new(self.album_id, self.album_title, alb_artist);

        Song {
            id: self.id,
            title: self.title,
            file_path: self.file_path,
            artist: artist,
            album: album,
        }
    }
}