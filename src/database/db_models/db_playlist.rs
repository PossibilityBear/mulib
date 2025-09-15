use sqlx::{prelude::FromRow};

use crate::models::playlist::Playlist;

#[derive(Debug, Clone, FromRow)]
pub struct DbPlaylist {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
}

impl Into<Playlist> for DbPlaylist {
    fn into(self) ->  Playlist {
        Playlist {
            id: self.id,
            title: self.title,
            description: self.description.unwrap_or(String::new()),
            songs: vec![],
        }
    }
}