use std::io;
use std::result;

use bincode::Error as BincodeError;
use serde_json::Error as SerdeError;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

pub type NikaResult<T, E = NikaError> = result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum NikaError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] SerdeError),
    #[error("Bincode error: {0}")]
    Bincode(#[from] BincodeError),
    #[error("Config error: {0}")]
    ConfigSerialization(#[from] TomlSerError),
    #[error("Config error: {0}")]
    ConfigDeserialization(#[from] TomlDeError),
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}
