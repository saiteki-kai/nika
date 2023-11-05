use std::path::PathBuf;

pub const APP_NAME: &str = "nika";

pub const WORDS_BIN_PATH: &str = "./data/jmdict-words.bin";
pub const TAGS_BIN_PATH: &str = "./data/jmdict-tags.bin";

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
