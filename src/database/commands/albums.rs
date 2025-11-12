use sqlx::Error;

use crate::database::db_models::*;
use crate::database::utils::db_connection::*;
use crate::models::album::Album;
use crate::models::artist::Artist;

/// blindly retreives all albums from the database
pub async fn get_all_albums(conn: &DbConnection) -> Result<Vec<Album>, Error> {
    let result = sqlx::query_as!(
        db_album::DbAlbum,
        "
        SELECT 
            alb.id,
            alb.title,
            alb.artist_id,
            art.name AS artist_name
        FROM Albums AS alb 
        LEFT JOIN Artists AS art on alb.artist_id = art.id
        ORDER BY 
            alb.title COLLATE NOCASE ASC
        "
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();


    let albums: Vec<Album> = result.into_iter().map(|res| { 
        res.into()
    }).collect();

    Ok(albums)
}