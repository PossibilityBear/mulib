
use std::vec;

use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::database::db_models::*;
use crate::models::album::Album;
use crate::models::artist::Artist;
use crate::database::utils::db_connection::*;
use crate::models::song::Song;

/// blindly retreives all songs from the database
/// for use in a 'dumb diff' approach to merging
/// songs from local music library into database.
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


    let songs: Vec<Song> = result.iter().map(|res| { 
        Song {
            id: res.id,
            title: res.title.clone(),
            file_path: res.file_path.clone(),
            artist: match (res.artist_id, res.artist_name.clone()) {
                (Some(id), Some(name)) => {
                    Some(Artist {
                        id: id,
                        name: name,
                    })
                },
                (_, _)=> None
            },
            album: match (res.album_id, res.album_title.clone()) {
                (Some(id), Some(title)) => {
                    Some(Album {
                        id: id,
                        title: title,
                        artist: match (res.album_artist_id, res.album_artist_name.clone()) {
                            (Some(id), Some(name)) => {
                                Some(Artist{
                                    id: id,
                                    name: name,
                                })
                            },
                            (_, _) => None
                        }
                    })
                },
                (_, _) => None,
            },
        }
    }).collect();

    Ok(songs)
}