use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use nika::core::config::*;
use nika::core::models::dictionary::{JMdict, Word};

fn parse_json() -> Result<JMdict> {
    let mut s = String::new();
    File::open(WORDS_JSON_PATH)
        .with_context(|| "Failed to open JSON data")?
        .read_to_string(&mut s)
        .with_context(|| "Failed to read JSON data")?;
    let json_data: JMdict =
        serde_json::from_str(&s).with_context(|| "Failed to parse JSON data")?;

    Ok(json_data)
}

fn generate_bincode(data: JMdict) -> Result<()> {
    let words: HashMap<String, Word> = data
        .words
        .into_par_iter()
        .map(|word| (word.id.clone(), word))
        .collect();

    let file = File::create(WORDS_BIN_PATH)?;
    let mut writer = std::io::BufWriter::new(file);
    bincode::serialize_into(&mut writer, &words).with_context(|| "Failed to serialize words")?;

    let file = File::create(TAGS_BIN_PATH)?;
    let mut writer = std::io::BufWriter::new(file);
    bincode::serialize_into(&mut writer, &data.tags).with_context(|| "Failed to serialize tags")?;

    Ok(())
}

fn run() -> Result<()> {
    println!("[1/4] Finding the latest release...");
    // TODO

    println!("[2/4] Downloading data...");
    // TODO

    println!("[3/4] Parsing data...");
    let data = parse_json()?;

    println!("[4/4] Generating binary files...");
    generate_bincode(data)?;

    println!("Update completed successfully.");

    Ok(())
}

fn main() {
    let cache_dir = app_cache_dir();

    let file_appender = tracing_appender::rolling::never(&cache_dir, "update.log");
    tracing_subscriber::fmt().with_writer(file_appender).init();

    if let Err(error) = run() {
        eprintln!(
            "Update failed. Please check the log file for more details at {:?}",
            cache_dir
        );
        tracing::error!("{}", error);
        std::process::exit(1);
    }

    std::process::exit(0);
}
