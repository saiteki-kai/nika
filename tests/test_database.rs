use std::fs;
use std::path::Path;

use nika::core::models::dictionary::JMdict;
use nika::core::repository::word_repository::WordRepository;

async fn setup_repo() -> WordRepository {
    let filepath = Path::new("data/test_words");

    // clear database
    if filepath.is_dir() {
        fs::remove_dir_all(filepath).expect("cannot delete directory");
    }

    let repo = WordRepository::new("data", "test_words").await;

    // load example data
    let json_str = include_str!("./examples/words1.json");
    let data: JMdict = serde_json::from_str(json_str).unwrap();

    // insert words in the database
    for word in data.words.iter() {
        repo.insert(word.clone()).await.unwrap();
    }

    repo
}

#[tokio::test]
async fn test_get_by_index() {
    let repo = setup_repo().await;

    // existing index
    let res = repo.get_by_index("1358280").unwrap();
    assert!(res.is_some());

    if let Some(word) = res {
        assert_eq!(word.id, "1358280");
    }

    // not existing index
    let res = repo.get_by_index("000000").unwrap();
    assert!(res.is_none());
}

// TODO: take care about async behavior for multiple tests
// TODO: look for a library for setup/teardown with async
