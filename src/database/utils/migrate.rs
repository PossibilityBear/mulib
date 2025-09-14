use crate::database::utils::db_connection::DbConnection;
use sqlx::migrate::Migrator;
use std::path::Path;

const MIGRATIONS_DIR: &str = "./src/migrations";

pub async fn migrate(conn: &DbConnection) {
    let crate_dir = "./";
    let migrations = Path::new(&crate_dir).join(MIGRATIONS_DIR);

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