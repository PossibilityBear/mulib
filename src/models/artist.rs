use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ParsedArtist {
    pub id: Option<i64>,
    pub name: String,
}

impl Into<ParsedArtist> for Artist {
    fn into(self) -> ParsedArtist {
        ParsedArtist { id: Some(self.id), name: self.name}
    }
}


impl Artist {
    /// creates a new artist or returns None if an artist
    /// cannot be constructed with given parameters
    pub fn new_or_none(id: Option<i64>, name: Option<String>) -> Option<Self> {
        match (id, name) {
            (Some(id), Some(name)) => {
                Some(Self {id, name})
            }
            (Some(id), None) => {
                Some(Self {id, name: String::new()})
            },
            (_, _) => None,
        }
    }
}