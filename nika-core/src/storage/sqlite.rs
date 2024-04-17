use std::path::Path;

use rusqlite::Connection;

use crate::errors::Result;

#[derive(Debug)]
pub struct Storage {
    pub db: Connection,
}

impl Storage {
    pub fn open<P: AsRef<Path>>(filepath: &P) -> Result<Self> {
        let connection = Connection::open(filepath)?;

        Self::from_connection(connection)
    }

    pub fn open_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;

        Self::from_connection(connection)
    }

    fn from_connection(connection: Connection) -> Result<Self> {
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.execute_batch(include_str!("schema.sql"))?;

        Ok(Self { db: connection })
    }
}
