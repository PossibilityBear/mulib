use sqlx::Error;

use crate::database::db_models::*;
use crate::database::utils::db_connection::*;
use crate::models::{album, artist};
use crate::models::song::Song;

/// blindly retreives all songs from the database
pub async fn get_all_songs(conn: &DbConnection) -> Result<Vec<Song>, Error> {
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


/// Retreive all songs for the given artist from the database
pub async fn get_songs_by_artist(conn: &DbConnection, artist_id: i64) -> Result<Vec<Song>, Error> {
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
        INNER JOIN Artists AS art ON art.Id = s.artist_id
        LEFT JOIN Albums AS alb ON alb.Id = s.album_id
        LEFT JOIN Artists AS AlbArt ON alb.artist_id= AlbArt.Id
        WHERE s.artist_id = ?
        ORDER BY 
            art.name COLLATE NOCASE ASC, 
            alb.title COLLATE NOCASE ASC, 
            s.title COLLATE NOCASE ASC 
        ",
        artist_id
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();


    let songs: Vec<Song> = result.into_iter().map(|res| { 
        res.into()
    }).collect();

    Ok(songs)
}

/// Retreive all songs for the given album from the database
pub async fn get_songs_by_album(conn: &DbConnection, album_id: i64) -> Result<Vec<Song>, Error> {
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
        INNER JOIN Albums AS alb ON alb.Id = s.album_id
        LEFT JOIN Artists AS art ON art.Id = s.artist_id
        LEFT JOIN Artists AS AlbArt ON alb.artist_id= AlbArt.Id
        WHERE s.album_id = ?
        ORDER BY 
            art.name COLLATE NOCASE ASC, 
            alb.title COLLATE NOCASE ASC, 
            s.title COLLATE NOCASE ASC 
        ",
        album_id
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();


    let songs: Vec<Song> = result.into_iter().map(|res| { 
        res.into()
    }).collect();

    Ok(songs)
}