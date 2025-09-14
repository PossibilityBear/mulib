use crate::database::utils::db_connection::DbConnection;
use sqlx::migrate::Migrator;
use std::path::Path;
use std::env;

const MIGRATIONS_PATH_ENV_KEY: &str = "DATABASE_MIRGRATIONS_PATH";

pub async fn migrate(conn: &DbConnection) {
    let mig_pat_str = &env::var(MIGRATIONS_PATH_ENV_KEY)
        .expect(&format!("To have found env var {}", MIGRATIONS_PATH_ENV_KEY));
    let migrations = Path::new(mig_pat_str);

    let migration_results = Migrator::new(migrations)
        .await
        .unwrap()   
        .run(&conn.db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    println!("migration: {:?}", migration_results);
}