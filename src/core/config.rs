use std::path::PathBuf;

use once_cell::sync::Lazy;

pub const APP_NAME: &str = "nika";

pub static WORDS_BIN_PATH: Lazy<PathBuf> =
    Lazy::new(|| app_cache_dir().join("data").join("jmdict-words.bin"));
pub static TAGS_BIN_PATH: Lazy<PathBuf> =
    Lazy::new(|| app_cache_dir().join("data").join("jmdict-tags.bin"));

pub const DICTIONARY_RELEASE_URL: &str =
    "https://api.github.com/repos/scriptin/jmdict-simplified/releases/latest";

pub fn app_cache_dir() -> PathBuf {
    let cache_dir: PathBuf = match dirs::cache_dir() {
        Some(path) => path.join(APP_NAME),
        None => {
            eprintln!("Could not determine cache directory.");
            std::process::exit(1);
        }
    };

    cache_dir
}

pub fn app_config_dir() -> PathBuf {
    let config_dir: PathBuf = match dirs::config_local_dir() {
        Some(path) => path.join(APP_NAME),
        None => {
            eprintln!("Could not determine config directory.");
            std::process::exit(1);
        }
    };

    config_dir
}
