use crate::models::playlist::Playlist;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DbPlaylist {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
}

impl Into<Playlist> for DbPlaylist {
    fn into(self) -> Playlist {
        Playlist::new(
            self.id,
            self.title,
            self.description.unwrap_or(String::new()),
            vec![],
        )
    }
}
