use sqlx::prelude::FromRow;

use crate::models::artist::Artist;

#[derive(Debug, Clone, FromRow)]
pub struct DbArtist {
    pub id:i64,
    pub name: String,
}


impl Into<Artist> for DbArtist {
    fn into(self) -> Artist {
        Artist {
            id: self.id,
            name: self.name
        }
    }
}