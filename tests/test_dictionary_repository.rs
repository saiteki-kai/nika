use nika::core::models::dictionary::JMdict;
use nika::core::repository::dictionary_repository::DictionaryRepository;
use nika::core::repository::dictionary_repository::TagMap;
use nika::core::repository::dictionary_repository::WordMap;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;

const WORDS: &str = include_str!("fixtures/words.json");

fn setup_repo() -> DictionaryRepository {
    let data: JMdict = serde_json::from_str(WORDS).unwrap();

    let words: WordMap = data
        .words
        .into_par_iter()
        .map(|word| (word.id.clone(), word))
        .collect();

    let tags: TagMap = data.tags;

    DictionaryRepository::from(words, tags)
}

#[test]
fn test_get_by_index() {
    let repo = setup_repo();

    // existing index
    let res = repo.word("1358280");
    assert!(res.is_some());

    if let Some(word) = res {
        assert_eq!(word.id, "1358280");
    }

    // not existing index
    let res = repo.word("0000");
    assert!(res.is_none());
}
