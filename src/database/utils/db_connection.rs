use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::env;

const DB_CONN_STR_ENV_KEY: &str = "DATABASE_URL";

#[derive(Clone, Debug)]
pub struct DbConnection {
    pub db: Pool<Sqlite>
}

impl DbConnection {
    pub async fn new() -> Self {
        let db_conn_str = &env::var(DB_CONN_STR_ENV_KEY)
            .expect(&format!("To have found env var {}", DB_CONN_STR_ENV_KEY));

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


