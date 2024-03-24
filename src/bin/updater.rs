use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use flate2::read::GzDecoder;
use nika::config::*;
use nika_core::models::jmdict::JMdict;
use nika_core::models::jmdict::Word;
use nika_core::models::kanjidic::Kanjidic;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use reqwest::header::USER_AGENT;
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use tar::Archive;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

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

    debug!("Download successful {}", url);

    // decompress
    let mut gz = GzDecoder::new(&content[..]);
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;

    debug!("Decompression successful");

    // extract
    let mut archive = Archive::new(&buffer[..]);

    if let Some(mut entry) = archive.entries()?.find_map(|e| e.ok()) {
        entry.unpack_in(destination_path)?;

        debug!("Extraction successful");

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

fn generate_bincode_kanjidic(data: Kanjidic) -> Result<()> {
    let file = File::create(KANJI_BIN_PATH.as_path())?;
    let mut writer = std::io::BufWriter::new(file);
    bincode::serialize_into(&mut writer, &data).with_context(|| "Failed to serialize kanji")?;

    Ok(())
}

fn generate_bincode_jmdict(data: JMdict) -> Result<()> {
    let words: HashMap<String, Word> = data
        .words
        .into_par_iter()
        .map(|word| (word.id.clone(), word))
        .collect();

    let file = File::create(WORDS_BIN_PATH.as_path())?;
    let mut writer = std::io::BufWriter::new(file);
    bincode::serialize_into(&mut writer, &words).with_context(|| "Failed to serialize words")?;

    let file = File::create(TAGS_BIN_PATH.as_path())?;
    let mut writer = std::io::BufWriter::new(file);
    bincode::serialize_into(&mut writer, &data.tags).with_context(|| "Failed to serialize tags")?;

    Ok(())
}

fn run() -> Result<()> {
    info!("[1/4] Finding the latest release...");
    let (jmdict_url, kanjidic_url) = find_release_url()?;

    let dest_dir = app_cache_dir().join("data");

    info!("[2/4] Downloading JMDict data...");
    let jmdict_path = download_and_extract_tgz(&jmdict_url, &dest_dir)?;

    info!("[2/4] Downloading Kanjidic2 data...");
    let kanjidic_path = download_and_extract_tgz(&kanjidic_url, &dest_dir)?;

    info!("[3/4] Parsing JMDict data...");
    let jmdict_data = parse_json::<JMdict>(&jmdict_path)?;

    info!("[3/4] Parsing Kanjidic2 data...");
    let kanjidic_data = parse_json::<Kanjidic>(&kanjidic_path)?;

    info!("[4/4] Generating JMDict binary...");
    generate_bincode_jmdict(jmdict_data)?;

    info!("[4/4] Generating Kanjidic2 binary...");
    generate_bincode_kanjidic(kanjidic_data)?;

    info!("Update completed successfully.");

    Ok(())
}

fn main() {
    let cache_dir = app_cache_dir();
    let file_appender = tracing_appender::rolling::daily(&cache_dir, "update.log");

    Registry::default()
        .with(
            fmt::Layer::new()
                .compact()
                .without_time()
                .with_level(false)
                .with_target(false)
                .with_writer(std::io::stdout)
                .with_filter(LevelFilter::INFO),
        )
        .with(
            fmt::Layer::new()
                .log_internal_errors(false)
                .with_file(false)
                .with_writer(file_appender)
                .with_filter(LevelFilter::DEBUG),
        )
        .init();

    if let Err(error) = run() {
        info!(
            "Update failed. Please check the log file for more details at {:?}",
            cache_dir
        );
        error!("{}", error);
        std::process::exit(1);
    }

    std::process::exit(0);
}
