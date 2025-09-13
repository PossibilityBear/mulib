use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DbAlbum {
    pub id: i64,
    pub title: String,
    pub artist_id: Option<i64>
}