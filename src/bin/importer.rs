use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use indicatif::ProgressBar;
use nika::config::app_cache_dir;
use nika::config::app_data_dir;
use nika::config::WORDS_BIN_PATH;
use nika_core::dictionary::WordMap;
use nika_core::importer::matching::Matcher;
use nika_core::importer::query::Query;
use nika_core::models::jmdict::Word;
use nika_core::models::study_list::StudyItem;
use rayon::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct ConflictResults {
    query: Query,
    results: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct MatchResult {
    matches: Vec<String>,
    conflicts: Vec<ConflictResults>,
    missing: Vec<Query>,
}

// TODO: need to keep the order in the matches when merging back the conflicts

fn find_matches(matcher: &Matcher, entries: Vec<Query>) -> MatchResult {
    let matches = Arc::new(Mutex::new(Vec::<String>::new()));
    let conflicts = Arc::new(Mutex::new(Vec::<ConflictResults>::new()));
    let missing = Arc::new(Mutex::new(Vec::<Query>::new()));

    let pb = ProgressBar::new(entries.len() as u64);

    entries.par_iter().for_each(|query| {
        let results = matcher.find(query, None, None);

        // if results.is_empty() && keep only common {
        //     results = matcher.find(query, None, Some(true));
        // }

        if results.is_empty() {
            missing.lock().unwrap().push(query.clone());
        } else if results.len() == 1 {
            matches
                .lock()
                .unwrap()
                .push(results.first().unwrap().id.clone());
        } else {
            let c = ConflictResults {
                query: query.to_owned(),
                results: results.iter().map(|r| r.id.clone()).collect(),
            };
            conflicts.lock().unwrap().push(c);
        }

        pb.inc(1);
    });

    pb.finish_with_message("done");

    let matches = matches.lock().unwrap().clone();
    let conflicts = conflicts.lock().unwrap().clone();
    let missing = missing.lock().unwrap().clone();

    println!(
        "Matches: {}, Conflicts: {}, Missing: {}",
        matches.len(),
        conflicts.len(),
        missing.len()
    );

    MatchResult {
        matches,
        conflicts,
        missing,
    }
}

type SenseMap = HashMap<String, HashSet<String>>;

fn main() -> Result<(), Box<dyn Error>> {
    // load the words
    let words = bincode::deserialize::<WordMap>(&fs::read(WORDS_BIN_PATH.as_path())?)?;
    let words: Vec<Word> = words.values().cloned().collect();
    println!("Words: {}", words.len());

    // load the senses map
    let senses_path = app_data_dir().join("senses.json");
    let senses_map = serde_json::from_str::<SenseMap>(&fs::read_to_string(senses_path)?)?;

    // load the queries
    let filepath = "./data/daily.json";
    // TODO: extract the context words for the query meanings
    let entries = serde_json::from_str::<Vec<Query>>(&fs::read_to_string(filepath)?)?;

    // do the matching
    let matcher = Matcher::new(words, senses_map);
    let match_results = find_matches(&matcher, entries);

    // save the results
    let conflict_path = app_cache_dir().join("conflicts.json");
    fs::write(conflict_path, serde_json::to_string(&match_results)?)?;

    Ok(())
}
