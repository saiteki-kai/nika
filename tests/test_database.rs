use anyhow::{Error, Result};
use std::env;
use std::fs;

use nika::core::models::dictionary::JMdict;
use nika::core::repository::word_repository::WordRepository;

const WORDS: &str = include_str!("fixtures/words.json");

async fn setup_repo() -> Result<WordRepository, Error> {
    let db_folder = env::temp_dir();
    let db_filepath = db_folder.join("test_words");

    // clear database
    if db_filepath.is_dir() {
        fs::remove_dir_all(&db_filepath).expect("cannot delete directory");
    }

    let repo = WordRepository::new(db_folder.to_str().unwrap(), "test_words").await;

    // load example data
    let data: JMdict = serde_json::from_str(WORDS)?;

    // insert words in the database
    for word in data.words.iter() {
        repo.insert(word.clone()).await.unwrap();
    }

    Ok(repo)
}

#[tokio::test]
async fn test_get_by_index() -> Result<(), Error> {
    let repo = setup_repo().await?;

    // existing index
    let res = repo.get_by_index("1358280").unwrap();
    assert!(res.is_some());

    if let Some(word) = res {
        assert_eq!(word.id, "1358280");
    }

    // not existing index
    let res = repo.get_by_index("000000").unwrap();
    assert!(res.is_none());

    Ok(())
}

// TODO: take care about async behavior for multiple tests
// TODO: look for a library for setup/teardown with async
