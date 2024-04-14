use std::io;
use std::result;

use bincode::Error as BincodeError;
use serde_json::Error as SerdeError;
use thiserror::Error;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

pub type Result<T, E = NikaError> = result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum NikaError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] SerdeError),
    #[error("Bincode error: {0}")]
    Bincode(#[from] BincodeError),
    #[error("List error: {0}")]
    List(#[from] StudyListError),
    #[error("Config error: {0}")]
    ConfigSerialization(#[from] TomlSerError),
    #[error("Config error: {0}")]
    ConfigDeserialization(#[from] TomlDeError),
}

#[derive(Error, Debug)]
pub enum StudyListError {
    #[error("List not found")]
    ListNotFound,
    #[error("List already exists")]
    ListAlreadyExists,
}
