use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use reqwest::header::USER_AGENT;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use tar::Archive;
use tracing::{error, info};

use nika::core::config::*;
use nika::core::models::dictionary::{JMdict, Word};

#[derive(Serialize, Deserialize, Debug)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]

struct Asset {
    name: String,
    size: u64,
    browser_download_url: String,
}

fn find_release_url() -> Result<(String, String)> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(DICTIONARY_RELEASE_URL)
        .header(USER_AGENT, APP_NAME)
        .send()
        .with_context(|| "Error retrieving data")?;

    let latest_release = response
        .json::<Release>()
        .with_context(|| "Error parsing JSON")?;

    let tgz_assets = latest_release
        .assets
        .iter()
        .filter(|asset| asset.name.ends_with(".tgz"))
        .collect::<Vec<&Asset>>();

    let jmdict_asset = tgz_assets
        .iter()
        .find(|asset| asset.name.contains("jmdict-eng") && !asset.name.contains("common"));

    let kanjidic_asset = tgz_assets
        .iter()
        .find(|asset| asset.name.contains("kanjidic2-en"));

    if let (Some(jmdict), Some(kanjidic)) = (jmdict_asset, kanjidic_asset) {
        if jmdict.size > 0 && kanjidic.size > 0 {
            return Ok((
                jmdict.browser_download_url.clone(),
                kanjidic.browser_download_url.clone(),
            ));
        }
        return Err(anyhow!("Empty resources"));
    }

    Err(anyhow!("URLs not found"))
}

fn download_and_extract_tgz(url: &str, destination_path: &PathBuf) -> Result<PathBuf> {
    // download
    let response = reqwest::blocking::get(url)?;
    let content = response.bytes()?;

    info!("Download successful {}", url);

    // decompress
    let mut gz = GzDecoder::new(&content[..]);
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;

    info!("Decompression successful {}", url);

    // extract
    let mut archive = Archive::new(&buffer[..]);

    if let Some(mut entry) = archive.entries()?.find_map(|e| e.ok()) {
        entry.unpack_in(destination_path)?;
        info!("Extraction successful {}", url);

        if let Ok(entry_path) = entry.path() {
            return Ok(destination_path.join(entry_path));
        }
    }

    Err(anyhow!("Could not extract the data"))
}

fn parse_json<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let mut s = String::new();
    File::open(path)
        .with_context(|| "Failed to open JSON data")?
        .read_to_string(&mut s)
        .with_context(|| "Failed to read JSON data")?;
    let json_data: T = serde_json::from_str(&s).with_context(|| "Failed to parse JSON data")?;

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
    let (jmdict_url, _) = find_release_url()?;

    let config_dir = app_config_dir();
    let dest_dir = config_dir.join("data");

    println!("[2/4] Downloading data...");
    let jmdict_path = download_and_extract_tgz(&jmdict_url, &dest_dir)?;
    // let kanjidic_path = download_and_extract_tgz(&kanjidic_url, &dest_dir)?;

    println!("[3/4] Parsing data...");
    let jmdict_data = parse_json::<JMdict>(&jmdict_path)?;

    println!("[4/4] Generating binary files...");
    generate_bincode(jmdict_data)?;

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
        error!("{}", error);
        std::process::exit(1);
    }

    std::process::exit(0);
}
