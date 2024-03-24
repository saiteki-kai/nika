use crate::errors::Result;
use crate::models::jmdict::Word;
use crate::models::link::Link;
use crate::models::study_list::StudyConfig;
use crate::models::study_list::StudyList;
use crate::repositories::config_repository::ConfigRepository;
use crate::repositories::dictionary_repository::DictionaryRepository;
use crate::repositories::list_repository::ListRepository;

pub struct StudyController {
    list_repository: ListRepository,
    dictionary_repository: DictionaryRepository,
    config_repository: ConfigRepository,
}

impl StudyController {
    pub fn new(
        dictionary_repository: DictionaryRepository,
        config_repository: ConfigRepository,
        list_repository: ListRepository,
    ) -> Self {
        Self {
            list_repository,
            dictionary_repository,
            config_repository,
        }
    }

    pub fn add(&self, study_list: StudyList) -> Result<()> {
        self.list_repository.add_list(study_list)
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        self.list_repository.remove_list(name)
    }

    pub fn list(&self, name: &str) -> Result<StudyList> {
        self.list_repository.get_list(name)
    }

    pub fn lists(&self) -> Result<Vec<StudyList>> {
        self.list_repository.get_lists()
    }

    pub fn study_words(&self, name: &str, daily: bool) -> Result<Vec<&Word>> {
        let study_list = self.list_repository.get_list(name)?;

        let index = study_list.config.current_index;
        let count = study_list.config.items_per_day;

        let id_list = study_list.items.iter();

        let ids: Vec<&str> = if !daily {
            id_list.map(AsRef::as_ref).collect()
        } else {
            id_list.skip(index).take(count).map(AsRef::as_ref).collect()
        };

        let words = self.dictionary_repository.words(&ids);

        Ok(words)
    }

    pub fn select(&self, name: &str) -> Result<()> {
        match self.list_repository.get_list(name) {
            Ok(_) => self.config_repository.set_current_list(name),
            Err(e) => Err(e),
        }
    }

    pub fn selected_list(&self) -> Result<Option<String>> {
        if let Some(name) = self.config_repository.get_current_list()? {
            return match self.list_repository.get_list(name.as_str()) {
                Ok(_) => Ok(Some(name)),
                Err(e) => Err(e),
            };
        }

        Ok(None)
    }

    pub fn update_config(&self, name: &str, config: StudyConfig) -> Result<()> {
        self.list_repository.update_list_config(name, config)
    }

    pub fn get_links(&self) -> Result<Vec<Link>> {
        self.config_repository.dictionaries()
    }
}
