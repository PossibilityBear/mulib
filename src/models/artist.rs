use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: Option<u32>,
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct ArtistDBModel {
    pub id: u32,
    pub name: String,
}