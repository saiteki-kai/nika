use std::path::PathBuf;

const APP_NAME: &str = "nika";

pub const WORDS_JSON_PATH: &str = "./data/jmdict-eng.json";
pub const WORDS_BIN_PATH: &str = "./data/jmdict-words.bin";
pub const TAGS_BIN_PATH: &str = "./data/jmdict-tags.bin";

pub fn app_cache_dir() -> String {
    let cache_dir: PathBuf = match dirs::cache_dir() {
        Some(path) => path.join(APP_NAME),
        None => {
            eprintln!("Could not determine cache directory.");
            std::process::exit(1);
        }
    };

    cache_dir.to_string_lossy().to_string()
}
