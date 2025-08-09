
use serde::{Deserialize, Serialize};

use crate::models::song::SongDBModel;
use crate::database::utils::db_connection::*;

pub fn get_songs(conn: DbConnection) -> Vec<SongDBModel> {
    let db = conn.db();
    let mut stmt = db.prepare(
        "
        SELECT id, title, filePath, artistId, albumId FROM Song
        "
    ).unwrap();
    let song_iter = stmt.query_map([], |row| {
        // println!("aritstId: {:?}", row.get::<usize, Option<u32>>(1));
        // Ok(())
        Ok(SongDBModel {
            id: row.get(0)?,
            title: row.get(1)?,
            file_path: row.get(2)?,
            artist_id: row.get::<usize, Option<u32>>(3)?,
            album_id: row.get::<usize, Option<u32>>(4)?,
        })
    }).unwrap();

    let songs: Vec<SongDBModel> = song_iter
        .map(|song| {
            song.unwrap()
        })
        .collect::<Vec<SongDBModel>>();

    songs
}