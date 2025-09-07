
use serde::{Deserialize, Serialize};

use crate::models::album::Album;
use crate::models::artist::Artist;
use crate::models::song::{Song, SongDBModel};
use crate::database::utils::db_connection::*;

pub fn get_songs(conn: DbConnection) -> Vec<Song> {
    let db = conn.db();
    let mut stmt = db.prepare(
        "
        SELECT 
            s.id, 
            s.title, 
            s.filePath, 
            art.id,
            art.name,
            alb.id,
            alb.title,
            AlbArt.id,
            AlbArt.name
        FROM Song AS s
        LEFT JOIN Album AS alb ON alb.Id = s.albumId
        LEFT JOIN Artist AS art ON art.Id = s.artistId
        LEFT JOIN Artist AS AlbArt ON alb.artistId = AlbArt.Id
        ORDER BY 
            art.name COLLATE NOCASE ASC, 
            alb.title COLLATE NOCASE ASC, 
            s.title COLLATE NOCASE ASC 
        "
    ).unwrap();


    let song_iter = stmt.query_map([], |row| {
        // println!("aritstId: {:?}", row.get::<usize, Option<u32>>(1));
        // Ok(())
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            file_path: row.get(2)?,
            artist: match row.get::<usize, Option<i32>>(3)? {
                Some(_) => {
                    Some(Artist {
                        id: row.get(3)?,
                        name: row.get(4)?,
                    })
                },
                None => None
            },
            album: match row.get::<usize, Option<i32>>(5)? {
                Some(_) => {
                    Some(Album {
                        id: row.get(5)?,
                        title: row.get(6)?,
                        artist: match row.get::<usize, Option<i32>>(7)? {
                            Some(_) => {
                                Some(Artist{
                                    id: row.get(7)?,
                                    name: row.get(8)?,
                                })
                            },
                            None => None
                        }
                    })
                },
                None => None,
            },
        })
    }).unwrap();

    let songs: Vec<Song> = song_iter
        .map(|song| {
            song.unwrap()
        })
        .collect::<Vec<Song>>();

    songs
}