use std::{clone, sync::{Arc, Mutex, MutexGuard}};
use rusqlite::{Connection, Result};


const DB_CONN_STR: &str = "./Music.db";

pub type Db<'a> = MutexGuard<'a, Connection>;

#[derive(Clone, Debug)]
pub struct DbConnection {
    db: Arc<Mutex<Connection>>
}

impl Default for DbConnection {
    fn default() -> Self {
        let db_conn = Connection::open(DB_CONN_STR).unwrap();
        let db = Arc::new(Mutex::new(db_conn));
        Self { db }
    }
}
impl DbConnection {
    pub fn db(&self) -> Db {
        self.db.lock().unwrap()
    }
}
