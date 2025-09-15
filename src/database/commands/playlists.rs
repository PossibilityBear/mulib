use sqlx::Error;

use crate::database::db_models::*;
use crate::database::utils::db_connection::*;
use crate::models::playlist::Playlist;
use crate::models::song::Song;

pub async fn get_playlist(conn: &DbConnection, list_id: &i64) -> Result<Playlist, Error> {
    let mut playlist = get_list(&conn, list_id).await?;
    let songs = get_list_songs(&conn, list_id).await?;
    playlist.songs = songs;
    Ok(playlist)

}

/// Retreives the list info itself
async fn get_list(conn: &DbConnection, list_id: &i64) -> Result<Playlist, Error> {
    let playlist = sqlx::query_as!(
        db_playlist::DbPlaylist,
        "
        SELECT 
            p.[id], 
            p.[title], 
            p.[description]
        FROM Playlists p  
        WHERE p.id = ?
        ",
        list_id
    )
        .fetch_one(&conn.db)
        .await
        .unwrap();

    Ok(playlist.into())
}

/// retreives all songs for the given play list
async fn get_list_songs(conn: &DbConnection, list_id: &i64) -> Result<Vec<Song>, Error> {
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
        FROM PlaylistSongs ps
        INNER JOIN Songs AS s ON  s.id = ps.song_id
        LEFT JOIN Albums AS alb ON alb.Id = s.album_id
        LEFT JOIN Artists AS art ON art.Id = s.artist_id
        LEFT JOIN Artists AS AlbArt ON alb.artist_id= AlbArt.Id
        WHERE ps.playlist_id = ?
        ORDER BY 
            art.name COLLATE NOCASE ASC, 
            alb.title COLLATE NOCASE ASC, 
            s.title COLLATE NOCASE ASC 
        ",
        list_id
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();


    let songs: Vec<Song> = result.into_iter().map(|res| { 
        res.into()
    }).collect();

    Ok(songs)
}


