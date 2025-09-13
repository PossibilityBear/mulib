use sqlx::prelude::FromRow;

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