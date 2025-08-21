use crate::models::artist::Artist;
use leptos_struct_table::*;
use leptos::prelude::*;

use serde::{Deserialize, Serialize};
#[derive(TableRow,Debug, Default, Clone, Serialize, Deserialize)]
#[table(impl_vec_data_provider)]
pub struct Album {
    pub id: Option<u32>,
    pub title: String,
    pub artist: Option<Artist>
}

impl CellValue for Album {
    type RenderOptions = ();

    fn render_value(self, options: Self::RenderOptions) -> impl IntoView {
        view!{<p>{self.title}</p>}
    }
}

#[derive(Debug, Clone)]
pub struct AlbumDBModel {
    pub id: u32,
    pub title: String,
    pub artist_id: Option<u32>
}

