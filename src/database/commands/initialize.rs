use std::collections::HashMap;

use rusqlite::{Connection, Result};
 
use crate::models::{
        album::{Album, AlbumDBModel}, 
        artist::{Artist, ArtistDBModel}, 
        song::Song
    };
    
use crate::database::utils::db_connection::*;


use crate::database::utils::song_import::read_music;
use crate::database::tables::*;

pub fn initialize_db(conn: DbConnection) -> Result<()> {
    
    artist::create_table(conn.clone())?;
    album::create_table(conn.clone())?;
    song::create_table(conn.clone())?;
    println!("Created Tables");
    
    // let mut db = conn.db();
    let songs = read_music("./music".into()).unwrap();
    
    let artists = init_artists(&songs, conn.clone())?;
    println!("Init'd Artists");
    let albums = init_albums(&songs, &artists, conn.clone())?;
    println!("Init'd Albums");
    init_songs(&songs, &artists, &albums, conn.clone())?;
    println!("Init'd Songs");
    Ok(())
}


// populate artist table from local music folder,
// returns a hashmap mapping artist names to their database id
fn init_artists(songs: &Vec<Song>, conn: DbConnection) -> Result<HashMap<String, u32>> {
    let mut db = conn.db();
    { // Initialize Artists
        let tx = db.transaction()?;
        {
            let mut artists = Vec::<Artist>::new();            
            for song in songs {
                if let Some(artist) = &song.artist {
                    let mut exists: bool = false;
                    for a in &artists {
                        if a.name == artist.name {
                            exists = true;
                            break;
                        } 
                    }
                    if exists {continue}
                    else {artists.push(artist.clone())}
                }
                if let Some(album) = &song.album {
                    if let Some(artist) = &album.artist {
                        let mut exists: bool = false;
                        for a in &artists {
                            if a.name == artist.name {
                                exists = true;
                                break;
                            } 
                        }
                        if exists {continue}
                        else {artists.push(artist.clone())}
                    }
                }
            }

            let mut stmt = tx.prepare("
                INSERT INTO Artist (name) VALUES (?)
            ")?;
            
            for artist in &artists {
                let _ = stmt.execute((&artist.name,))?;
            }
        }
        tx.commit()?;
    }
    // get artists with assigned Id's from db
    let mut artists: HashMap<String, u32> = HashMap::new();
    
    let mut stmt = db.prepare("
        SELECT id, name FROM artist 
    ")?;
    let artist_iter = stmt.query_map([], |row| {
        Ok(ArtistDBModel {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?; 

    for artist in artist_iter {
        let a = artist.unwrap();
        artists.insert(a.name, a.id);
    }
    Ok(artists)
}


// initialize all albums from local file structure
// return a hash map with album titles and their db id
fn init_albums(songs: &Vec<Song>, artists: &HashMap<String, u32>, conn: DbConnection) -> Result<HashMap<String, u32>>{
    let mut db = conn.db();
    { // Initialize Albums
        let tx = db.transaction()?;
        {
            let mut albums = Vec::<Album>::new();            
            for song in songs {
                if let Some(album) = &song.album {
                    let mut exists: bool = false;
                    for a in &albums {
                        if a.title == album.title {
                            exists = true;
                            break;
                        } 
                    }
                    if exists {continue}
                    else {albums.push(album.clone())}
                }
            }

            let mut stmt = tx.prepare("
                INSERT INTO Album (title, artistId) VALUES (?, ?)
            ")?;
            
            for album in &albums {
                let artist_id = artists.get(&album.artist.clone().unwrap().name).unwrap();
                let _ = stmt.execute((album.title.clone(), artist_id))?;
            }
        }
        tx.commit()?;
    }


    // get albums with associated Id's from db
    let mut albums: HashMap<String, u32> = HashMap::new();
    let mut stmt = db.prepare("
        SELECT id, title, artistId FROM album 
    ")?;
    let album_iter = stmt.query_map([], |row| {
        Ok(AlbumDBModel {
            id: row.get(0)?,
            title: row.get(1)?,
            artist_id: row.get(2)?,
        })
    })?; 
    for album in album_iter {
        let a = album.unwrap();
        albums.insert(a.title, a.id);
    }
    Ok(albums)
}


fn init_songs(songs: &Vec<Song>, artists: &HashMap<String, u32>, albums: &HashMap<String, u32>, conn: DbConnection) -> Result<()>{
    let mut db = conn.db();
    let tx = db.transaction()?;
    {
        let mut stmt = tx.prepare("
            INSERT INTO Song (title, filePath, artistId, albumId) VALUES (?, ?, ?, ?)
        ")?;

        for song in songs {
            let artist_id: Option<u32> = match &song.artist {
                Some(artist) => {
                    Some(*artists.get(&artist.name).unwrap())
                }
                None => None
            };

            let album_id: Option<u32> = match &song.album {
                Some(album) => {
                    Some(*albums.get(&album.title).unwrap())
                }
                None => None
            };

            // let artist_id = artists.get(&song.artist.name);
            // let album_id = artists.get(&song.album.unwrap().title);
            let _ = stmt.execute((&song.title, &song.file_path, artist_id, album_id))?;
        }
    }
    tx.commit()?;
    Ok(())
}
