use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use sqlx::{Pool, Sqlite};

use crate::database::utils::db_connection::DbConnection;


#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: DbConnection
}