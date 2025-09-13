use std::collections::HashMap;
use sqlx::Error;
use crate::{database::{db_models::*, utils::{get_all_songs::get_all_songs, migrate::migrate}}, models::{
        album::{Album, ParsedAlbum}, 
        artist::{Artist, ParsedArtist}, 
        song::{ParsedSong, Song}
    }};
use crate::database::utils::db_connection::*;
use crate::database::utils::song_import::read_music;

pub async fn initialize_db(conn: &DbConnection) -> Result<(), Error> {
    let local_songs = read_music("./music".into()).unwrap();
    println!("Read {} Songs From Local Library", local_songs.len());

    let db_songs: Vec<ParsedSong> = get_all_songs(conn).await?.into_iter()
        .map(|s| s.into())
        .collect();

    let (new, _updated, _deleted) = diff_songs(local_songs, db_songs.into());

    let artists = init_artists(&new, conn).await?;
    println!("Initialized Artists");

    let albums = init_albums(&new, &artists, conn).await?;
    println!("Initialized Albums");

    init_songs(&new, &artists, &albums, conn).await?;
    println!("Initialized Songs");
    Ok(())
}

/// compares the vec of locals songs to the vec of songs from database
/// Returns a tuple of song vecs (New, Updated, Removed)
fn diff_songs(local: Vec<ParsedSong>, db: Vec<ParsedSong>) -> (Vec<ParsedSong>, Vec<ParsedSong>, Vec<ParsedSong>) {
    let new: Vec<ParsedSong> = local.iter()
        .filter(|l| {
            None == db.iter().find(|d| d.file_path == l.file_path)
        })
        .map(|l| l.clone())
        .collect();
    
    let updated: Vec<ParsedSong> = local.iter()
        .filter(|l| !db.contains(l))
        .filter(|l| !new.contains(l))
        .map(|l| l.clone())
        .collect();
    
    let removed: Vec<ParsedSong> = db.into_iter()
        .filter(|d| {
            None == local.iter().find(|l| l.file_path == d.file_path)
        })
        .collect();

    (new, updated, removed)
}


// populate artist table from local music folder,
// returns a hashmap mapping artist names to their database id
async fn init_artists(songs: &Vec<ParsedSong>, conn: &DbConnection) -> Result<HashMap<String, i64>, Error> {
    { // Initialize Artists
        let mut artists = Vec::<ParsedArtist>::new();            
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


        let mut tx =conn.db.begin().await?;
        for artist in &artists {
            _ = sqlx::query!(
                "
                INSERT INTO Artists (name) VALUES (?) 
                ",
                artist.name
            )
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
    }
    // get artists with assigned Id's from db
    let mut artists: HashMap<String, i64> = HashMap::new();
    
    let result= sqlx::query_as!(
        db_artist::DbArtist,
        "
        SELECT id, name FROM Artists 
        "
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();

    for artist in result.iter() {
        let a = artist;
        artists.insert(a.name.clone(), a.id);
    }
    Ok(artists)
}


// initialize all albums from local file structure
// return a hash map with album titles and their db id
async fn init_albums(songs: &Vec<ParsedSong>, artists: &HashMap<String, i64>, conn: &DbConnection) -> Result<HashMap<String, i64>, Error>{
    { // Initialize Albums
        let mut albums = Vec::<ParsedAlbum>::new();            
        for song in songs {
            if let Some(album) = &song.album {
                // check for existing album with this name
                // skip duplicates
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

        let mut tx = conn.db.begin().await?;
        for album in &albums {
            let artist_id = 
            if let Some(artist) = &album.artist{
                artists.get(&artist.name)
            } else {
                None
            };

            _ = sqlx::query!(
                "
                INSERT INTO Albums (title, artist_id) VALUES (?, ?)
                ",
                album.title,
                artist_id
            )
                .execute(&mut  *tx)
                .await?;
        }
        tx.commit().await?;
    }


    // get albums with associated Id's from db
    let mut albums: HashMap<String, i64> = HashMap::new();
    let result = sqlx::query_as!(
        db_album::DbAlbum,
        "
        SELECT id, title, artist_id FROM Albums
        "
    )
        .fetch_all(&conn.db)
        .await
        .unwrap();

    for album in result.iter() {
        let a = album;
        albums.insert(a.title.clone(), a.id);
    }
    Ok(albums)
}


async fn init_songs(songs: &Vec<ParsedSong>, artists: &HashMap<String, i64>, albums: &HashMap<String, i64>, conn: &DbConnection) -> Result<(), Error>{
    let mut tx = conn.db.begin().await?; 
    for song in songs {
        let artist_id: Option<i64> = match &song.artist {
            Some(artist) => {
                Some(*artists.get(&artist.name).unwrap())
            }
            None => None
        };

        let album_id: Option<i64> = match &song.album {
            Some(album) => {
                Some(*albums.get(&album.title).unwrap())
            }
            None => None
        };

        _ = sqlx::query!(
            "
            INSERT INTO Songs (title, file_path, artist_id, album_id) VALUES (?, ?, ?, ?)
            ",
            song.title,
            song.file_path,
            artist_id,
            album_id
        )
            .execute(&mut *tx)
            .await?
            ;
    }
    tx.commit().await?; 
    Ok(())
}
