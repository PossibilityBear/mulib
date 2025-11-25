use crate::config_keys;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::env;

#[derive(Clone, Debug)]
pub struct DbConnection {
    pub db: Pool<Sqlite>,
}

impl DbConnection {
    pub async fn new() -> Self {
        let db_conn_str = &env::var(config_keys::DATABSE_URL).expect(&format!(
            "To have found env var {}",
            config_keys::DATABSE_URL
        ));

        if !Sqlite::database_exists(db_conn_str).await.unwrap_or(false) {
            println!("Creating database {}", db_conn_str);
            match Sqlite::create_database(db_conn_str).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }

        let db = SqlitePool::connect(db_conn_str).await.unwrap();
        Self { db }
    }
}
