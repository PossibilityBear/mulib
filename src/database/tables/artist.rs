use rusqlite::{Connection, Result};

use crate::database::utils::db_connection::*;

pub fn create_table(conn: DbConnection) -> Result<()>{
    let mut db = conn.db();
    db.execute_batch(
        "
        DROP TABLE IF EXISTS Artist;
        CREATE TABLE Artist (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
        "
    )?;
    Ok(())
}