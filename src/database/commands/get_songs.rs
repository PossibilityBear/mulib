
use sqlx::Error;

use crate::database::db_models::*;
use crate::database::utils::db_connection::*;
use crate::models::song::Song;

pub async fn get_songs(conn: &DbConnection) -> Result<Vec<Song>, Error> {
    let result = sqlx::query_as!(
        db_song::DbSong,
        "
        SELECT 
            s.id, 
            s.title, 
            s.file_path, 
            art.id AS artist_id,
            art.name AS artist_name,
            alb.id AS album_id,
            alb.title AS album_title,
            AlbArt.id AS album_artist_id,
            AlbArt.name AS album_artist_name
        FROM Songs AS s
        LEFT JOIN Albums AS alb ON alb.Id = s.album_id
        LEFT JOIN Artists AS art ON art.Id = s.artist_id
        LEFT JOIN Artists AS AlbArt ON alb.artist_id= AlbArt.Id
        ORDER BY 
            art.name COLLATE NOCASE ASC, 
            alb.title COLLATE NOCASE ASC, 
            s.title COLLATE NOCASE ASC 
        "
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();


    let songs: Vec<Song> = result.into_iter().map(|res| { 
        res.into()
    }).collect();

    Ok(songs)
}