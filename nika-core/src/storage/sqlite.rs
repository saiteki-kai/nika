use std::path::Path;

use rusqlite::Connection;

use crate::errors::NikaResult;

#[derive(Debug)]
pub struct Storage {
    pub(super) db: Connection,
}

impl Storage {
    pub fn open<P: AsRef<Path>>(filepath: &P) -> NikaResult<Self> {
        let connection = Connection::open(filepath)?;

        Self::from_connection(connection)
    }

    pub fn open_in_memory() -> NikaResult<Self> {
        let connection = Connection::open_in_memory()?;

        Self::from_connection(connection)
    }

    fn from_connection(connection: Connection) -> NikaResult<Self> {
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.execute_batch(include_str!("schema.sql"))?;

        Ok(Self { db: connection })
    }

    pub fn clear(&self) -> NikaResult<()> {
        self.db.execute_batch("DELETE FROM study_item_progress;")?;
        self.db.execute_batch("DELETE FROM daily_list;")?;
        self.db.execute_batch("DELETE FROM discovery_list;")?;
        self.db.execute_batch("DELETE FROM study_statistics;")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_open_in_memory() {
        Storage::open_in_memory().expect("Failed to open storage");
    }

    #[test]
    fn test_open_from_file() {
        let tempdir = tempdir().expect("Failed to create temporary directory");
        let filepath = tempdir.path().join("storage.db");

        Storage::open(&filepath).expect("Failed to open storage");
        assert!(&filepath.exists());

        fs::remove_dir_all(tempdir).expect("Failed to remove temporary directory");
    }
}
