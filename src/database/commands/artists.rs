use sqlx::Error;

use crate::database::db_models::*;
use crate::database::utils::db_connection::*;
use crate::models::artist::Artist;

/// blindly retreives all artists from the database
pub async fn get_all_artists(conn: &DbConnection) -> Result<Vec<Artist>, Error> {
    let result = sqlx::query_as!(
        db_artist::DbArtist,
        "
        SELECT 
            art.id,
            art.name
        FROM Artists AS art 
        ORDER BY 
            art.name COLLATE NOCASE ASC
        "
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();


    let artists: Vec<Artist> = result.into_iter().map(|res| { 
        res.into()
    }).collect();

    Ok(artists)
}