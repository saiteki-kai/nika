use core::fmt;
use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;

pub const APP_NAME: &str = "nika";
pub const CONFIG_NAME: &str = "config";

pub static WORDS_BIN_PATH: Lazy<PathBuf> = Lazy::new(|| app_data_dir().join("jmdict-words.bin"));
pub static TAGS_BIN_PATH: Lazy<PathBuf> = Lazy::new(|| app_data_dir().join("jmdict-tags.bin"));
pub static KANJI_BIN_PATH: Lazy<PathBuf> = Lazy::new(|| app_data_dir().join("kanjidic.bin"));

pub const DICTIONARY_RELEASE_URL: &str =
    "https://api.github.com/repos/scriptin/jmdict-simplified/releases/latest";

enum DirectoryType {
    Cache,
    Config,
    Data,
}

impl fmt::Display for DirectoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DirectoryType::Cache => write!(f, "cache"),
            DirectoryType::Config => write!(f, "config"),
            DirectoryType::Data => write!(f, "data"),
        }
    }
}

fn app_dir(dir_type: DirectoryType) -> PathBuf {
    let dir: Option<PathBuf> = match dir_type {
        DirectoryType::Cache => dirs::cache_dir(),
        DirectoryType::Config => dirs::config_local_dir(),
        DirectoryType::Data => dirs::data_local_dir(),
    };

    let path: PathBuf = match dir {
        Some(path) => path.join(APP_NAME),
        None => {
            eprintln!("Could not determine {} directory.", dir_type);
            std::process::exit(1);
        }
    };

    fs::create_dir_all(&path).ok();

    path
}

pub fn app_cache_dir() -> PathBuf {
    app_dir(DirectoryType::Cache)
}

pub fn app_config_dir() -> PathBuf {
    app_dir(DirectoryType::Config)
}

pub fn app_data_dir() -> PathBuf {
    app_dir(DirectoryType::Data)
}
