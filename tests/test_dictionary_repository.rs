use nika::core::dictionary::Dictionary;
use nika::core::dictionary::TagMap;
use nika::core::dictionary::WordMap;
use nika::core::models::jmdict::JMdict;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;

const WORDS: &str = include_str!("fixtures/words.json");

fn setup_repo() -> Dictionary {
    let data: JMdict = serde_json::from_str(WORDS).unwrap();

    let words: WordMap = data
        .words
        .into_par_iter()
        .map(|word| (word.id.clone(), word))
        .collect();

    let tags: TagMap = data.tags;

    Dictionary::from(words, tags)
}

mod get_word_by_id {
    use super::setup_repo;

    #[test]
    fn test_existing_id() {
        let repo = setup_repo();

        let res = repo.word("1358280");
        assert!(res.is_some());

        if let Some(word) = res {
            assert_eq!(word.id, "1358280");
        }
    }

    #[test]
    fn test_non_existing_id() {
        let repo = setup_repo();

        let res = repo.word("9999999");
        assert!(res.is_none());
    }
}

mod get_words {
    use super::setup_repo;

    #[test]
    fn test_all_existing_ids() {
        let repo = setup_repo();

        let ids: Vec<&str> = vec!["1008590", "1318720"];
        let n_elements = ids.len();

        let res = repo.words(&ids);
        assert_eq!(res.len(), n_elements);

        for word in res.clone() {
            assert!(ids.contains(&word.id.as_str()));
        }
    }

    #[test]
    fn test_one_existing_id() {
        let repo = setup_repo();

        let ids: Vec<&str> = vec!["0000000", "1318720", "0000001"];

        let res = repo.words(&ids);
        assert_eq!(res.len(), 1);

        assert!(ids.contains(&res.first().unwrap().id.as_str()));
    }

    #[test]
    fn test_all_non_existing_ids() {
        let repo = setup_repo();

        let ids: Vec<&str> = vec!["0000000", "0000001"];

        let res = repo.words(&ids);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_empty() {
        let repo = setup_repo();

        let ids: Vec<&str> = vec![];

        let res = repo.words(&ids);
        assert_eq!(res.len(), 0);
    }
}

mod random_words {
    use super::setup_repo;

    #[test]
    fn test_zero_random_words() {
        let repo = setup_repo();

        let res = repo.random_words(0);
        assert!(res.is_empty());
    }

    #[test]
    fn test_one_random_word() {
        let repo = setup_repo();

        let res = repo.random_words(1);
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_duplicated_words() {
        let repo = setup_repo();

        let res = repo.random_words(4);
        assert_eq!(res.len(), 4);

        let mut ids: Vec<String> = res.iter().map(|word| word.id.clone()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 4);
    }

    #[test]
    fn test_more_than_total_words() {
        let repo = setup_repo();
        let total = repo.num_words();

        let res = repo.random_words(total + 1);
        assert_eq!(res.len(), total);
    }
}
