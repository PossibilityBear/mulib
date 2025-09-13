use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DbArtist {
    pub id:i64,
    pub name: String,
}