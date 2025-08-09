use std::fs::*;
use std::io;
use std::path::*;
use id3::{
    frame::{
        Chapter, Comment, EncapsulatedObject, ExtendedLink, ExtendedText, InvolvedPeopleList,
        InvolvedPeopleListItem, Lyrics, Picture, Popularimeter, SynchronisedLyrics,
        UniqueFileIdentifier,
    },
    Content, Tag,
};

use crate::models::album::Album;
use crate::models::artist::Artist;
use crate::models::song::Song;

pub fn read_music(path: PathBuf) -> io::Result<Vec<Song>>{
    let dir = read_dir(path)?;
    let mut songs = Vec::<Song>::new();
    for entry in dir {
        let e = entry?;
        if !e.path().is_dir() {
            let file = File::open(e.path())?;
            let tag = Tag::read_from2(file).unwrap();

            let mut album: Option<Album> = None;
            let mut contributing_artist: Option<Artist> = None;
            let mut album_artist: Option<Artist> = None;
            let mut title: String = e.file_name().into_string().unwrap();

            for frame in tag.frames() {
                let id = frame.id();
                match frame.content() {
                    Content::Text(value) | Content::Link(value) => {
                        match id {
                            "TPE1" => {
                                //contributing artist
                                contributing_artist = Some(Artist {
                                    id: None,
                                    name: value.to_string(),
                                })
                            }
                            "TPE2" => {
                                //album artist
                                album_artist = Some(Artist {
                                    id: None,
                                    name: value.to_string(),
                                })
                            }
                            "TALB" => {
                                //album 
                                album = Some(Album {
                                    id: None,
                                    title: value.to_string(),
                                    artist: None,
                                })
                            }
                            "TIT2" => {
                                //song title
                                title = value.to_string();
                            }
                            _ => {}
                        }
                    }
                    Content::Picture(Picture {
                        mime_type,
                        picture_type,
                        description,
                        data,
                    }) => {
                        let size = data.len();
                        println!("{id}:{picture_type}=<image, {mime_type}, description {description:?}, {size} bytes>");
                    }
                    Content::UniqueFileIdentifier(UniqueFileIdentifier {
                        owner_identifier,
                        identifier,
                    }) => {
                        let value = identifier
                            .iter()
                            .map(|&byte| {
                                char::from_u32(byte.into())
                                    .map(|c| String::from(c))
                                    .unwrap_or_else(|| format!("\\x{:02X}", byte))
                            })
                            .collect::<String>();
                        println!("{id}:{owner_identifier}=b\"{value}\"");
                    }
                    _ => {}
                }
            }
            if let Some(ref mut album) = album {
                album.artist = album_artist;
            }
            
            songs.push(
                Song {
                    id: None,
                    title: title,
                    file_path: e.path().as_mut_os_string().clone().into_string().unwrap(),
                    artist: contributing_artist,
                    album: album
            })
        } else {
            songs.append(&mut read_music(e.path()).unwrap());
        }
    };
    Ok(songs)
}