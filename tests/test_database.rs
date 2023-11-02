use std::fs;
use std::path::Path;

use nika::core::models::dictionary::JMdict;
use nika::core::repository::word_repository::WordRepository;

static ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");

async fn setup_repo() -> WordRepository {
    let db_folder = Path::new(ROOT_DIR).join("data");
    let db_filepath = db_folder.join("test_words");

    // clear database
    if db_filepath.is_dir() {
        fs::remove_dir_all(&db_filepath).expect("cannot delete directory");
    }

    let repo = WordRepository::new(db_folder.to_str().unwrap(), "test_words").await;

    // load example data
    let examples_path = Path::new(ROOT_DIR).join("tests").join("examples");
    let result = fs::read_to_string(examples_path.join("words1.json"));

    match result {
        Ok(json_str) => {
            let data: JMdict = serde_json::from_str(&json_str).unwrap();

            // insert words in the database
            for word in data.words.iter() {
                repo.insert(word.clone()).await.unwrap();
            }
        }
        Err(error) => eprintln!("Error reading file: {}", error),
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
