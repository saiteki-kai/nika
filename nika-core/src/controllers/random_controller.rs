use crate::models::jmdict::Word;
use crate::repositories::dictionary_repository::DictionaryRepository;

pub struct RandomController {
    dictionary_repository: DictionaryRepository,
}

impl RandomController {
    pub fn new(dictionary_repository: DictionaryRepository) -> Self {
        Self {
            dictionary_repository,
        }
    }

    pub fn random_words(&self, count: usize) -> Vec<&Word> {
        self.dictionary_repository.random_words(count)
    }
}
