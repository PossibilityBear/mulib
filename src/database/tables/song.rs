use rusqlite::{Connection, Result};
use crate::database::utils::db_connection::*;

pub fn create_table(conn: DbConnection) -> Result<()> {
    let mut db = conn.db();
    db.execute_batch(
        "
        DROP TABLE IF EXISTS Song;
        CREATE TABLE Song (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            filePath TEXT NOT NULL,
            albumId INTEGER,
            artistId INTEGER
        );
        "
    )?;

    Ok(())
}