#[cfg(feature = "ssr")]
pub mod app_state;
pub mod components;
pub mod config_keys;
pub mod database;
pub mod models;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::components::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
