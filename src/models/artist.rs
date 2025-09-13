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