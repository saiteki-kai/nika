use crate::models::jmdict::Word;
use crate::repositories::dictionary_repository::DictionaryRepository;

pub struct SearchController {
    dictionary_repository: DictionaryRepository,
}

impl SearchController {
    pub fn new(dictionary_repository: DictionaryRepository) -> Self {
        Self {
            dictionary_repository,
        }
    }

    pub fn search(&self, query: &str, common: Option<bool>) -> Vec<Word> {
        self.dictionary_repository.search(query, common)
    }
}
