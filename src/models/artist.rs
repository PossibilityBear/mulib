use serde::{Deserialize, Serialize};
use leptos_struct_table::*;
use leptos::prelude::*;


#[derive(TableRow, Debug, Default, Clone, Serialize, Deserialize)]
#[table(impl_vec_data_provider)]
pub struct Artist {
    pub id: Option<u32>,
    pub name: String,
}


impl CellValue for Artist {
    type RenderOptions = ();

    fn render_value(self, options: Self::RenderOptions) -> impl IntoView {
        view! {<p>{self.name}</p>}
    }
}
#[derive(Debug, Clone)]
pub struct ArtistDBModel {
    pub id: u32,
    pub name: String,
}