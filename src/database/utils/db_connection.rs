// use std::{clone, sync::{Arc, Mutex, MutexGuard}};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};


const DB_CONN_STR: &str = "sqlite://music.db";

#[derive(Clone, Debug)]
pub struct DbConnection {
    pub db: Pool<Sqlite>
}

impl DbConnection {
    pub async fn new() -> Self {
        if !Sqlite::database_exists(DB_CONN_STR).await.unwrap_or(false) {
            println!("Creating database {}", DB_CONN_STR);
            match Sqlite::create_database(DB_CONN_STR).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }

        let db = SqlitePool::connect(DB_CONN_STR).await.unwrap();
        Self { db }
    }
}


