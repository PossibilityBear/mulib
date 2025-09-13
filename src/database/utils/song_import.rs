use std::fs::*;
use std::io;
use std::path::*;
use id3::{
    frame::{
        Picture, 
        UniqueFileIdentifier,
    },
    Content, Tag,
};

use crate::models::album::ParsedAlbum;
use crate::models::artist::ParsedArtist;
use crate::models::song::ParsedSong;

/// Recursively reads through provided dir 
/// and parses metadata on all songs found
pub fn read_music(path: PathBuf) -> io::Result<Vec<ParsedSong>>{
    let dir = read_dir(path)?;
    let mut songs = Vec::<ParsedSong>::new();
    for entry in dir {
        let e = entry?;
        if !e.path().is_dir() {
            let file = File::open(e.path())?;
            let Ok(tag) = Tag::read_from2(file) else {
                continue;
            };


            let mut album: Option<ParsedAlbum> = None;
            let mut contributing_artist: Option<ParsedArtist> = None;
            let mut album_artist: Option<ParsedArtist> = None;
            let mut title: String = e.file_name().into_string().expect("file name to be String compatible");

            for frame in tag.frames() {
                let id = frame.id();
                match frame.content() {
                    Content::Text(value) | Content::Link(value) => {
                        match id {
                            "TPE1" => {
                                //contributing artist
                                contributing_artist = Some(ParsedArtist {
                                    id: None,
                                    name: value.to_string(),
                                })
                            }
                            "TPE2" => {
                                //album artist
                                album_artist = Some(ParsedArtist {
                                    id: None,
                                    name: value.to_string(),
                                })
                            }
                            "TALB" => {
                                //album 
                                album = Some(ParsedAlbum {
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
                        mime_type: _,
                        picture_type: _,
                        description: _,
                        data,
                    }) => {
                        let size = data.len();
                        // println!("{id}:{picture_type}=<image, {mime_type}, description {description:?}, {size} bytes>");
                    }
                    Content::UniqueFileIdentifier(UniqueFileIdentifier {
                        owner_identifier: _,
                        identifier,
                    }) => {
                        let _value = identifier
                            .iter()
                            .map(|&byte| {
                                char::from_u32(byte.into())
                                    .map(|c| String::from(c))
                                    .unwrap_or_else(|| format!("\\x{:02X}", byte))
                            })
                            .collect::<String>();
                        // println!("{id}:{owner_identifier}=b\"{value}\"");
                    }
                    _ => {}
                }
            }
            if let Some(ref mut album) = album {
                album.artist = album_artist;
            }
            
            songs.push(
                ParsedSong {
                    id: None,
                    title: title,
                    file_path: e.path().as_mut_os_string().clone().into_string().expect("file path to be String compatible"),
                    artist: contributing_artist,
                    album: album
            })
        } else {
            songs.append(&mut read_music(e.path()).expect("to have read music from disk"));
        }
    };
    Ok(songs)
}