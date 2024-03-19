use crate::core::errors::Result;
use crate::core::models::jmdict::Word;
use crate::core::models::link::Link;
use crate::core::models::study_list::StudyConfig;
use crate::core::models::study_list::StudyList;
use crate::core::repositories::config_repository::ConfigRepository;
use crate::core::repositories::dictionary_repository::DictionaryRepository;
use crate::core::repositories::list_repository::ListRepository;

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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::*;
    use crate::core::errors::ErrorKind;
    use crate::core::errors::StudyListError;

    fn setup() -> StudyController {
        let tmp_dir = tempdir().unwrap();

        let lists_path = tmp_dir.into_path();
        let config_filepath = tempfile::NamedTempFile::new().unwrap().path().to_path_buf();

        let list_repository = ListRepository::new(lists_path);
        let config_repository = ConfigRepository::new(config_filepath);
        let dictionary_repository = DictionaryRepository::new().unwrap();

        StudyController::new(dictionary_repository, config_repository, list_repository)
    }

    fn get_fixture(name: &str) -> PathBuf {
        if name != "list1" && name != "list2" {
            eprintln!("The only lists availables are 'list1' and 'list2'");
        }

        let fixtures_path = Path::new("tests").join("fixtures").join("study_list");

        fixtures_path.join(name)
    }

    fn read_fixture_items(name: &str) -> Vec<String> {
        let list = get_fixture(name);

        let items = fs::read_to_string(list)
            .unwrap()
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();

        items
    }

    #[test]
    fn test_initialization() {
        let study_controller = setup();

        let lists = study_controller.lists().unwrap();
        assert_eq!(lists.len(), 0);
    }

    #[test]
    fn test_valid_add() {
        let controller = setup();

        let name = "list1";
        let items = read_fixture_items(name);
        let study_list = StudyList::new(name, items.clone());

        let result = controller.add(study_list.clone());
        assert!(result.is_ok());

        assert_eq!(controller.list(name).unwrap(), study_list);
        assert_eq!(controller.lists().unwrap().len(), 1);
        assert!(controller.selected_list().unwrap().is_none());
    }

    #[test]
    fn test_duplicated_add() {
        let controller = setup();

        let name = "list1";

        let items1 = read_fixture_items(name);
        let study_list1 = StudyList::new(name, items1.clone());

        let items2 = read_fixture_items(name);
        let study_list2 = StudyList::new(name, items2.clone());

        let result1 = controller.add(study_list1.clone());
        let result2 = controller.add(study_list2);

        assert!(result1.is_ok());
        assert!(matches!(
            result2.unwrap_err(),
            ErrorKind::List(StudyListError::ListAlreadyExists)
        ));

        let lists = controller.lists().unwrap();
        assert_eq!(lists.len(), 1);

        assert_eq!(controller.list(name).unwrap(), study_list1);
    }

    #[test]
    fn test_remove() {
        let controller = setup();

        let name1 = "list1";
        let items1 = read_fixture_items(name1);
        let study_list1 = StudyList::new(name1, items1);
        controller.add(study_list1).unwrap();

        let name2 = "list2";
        let items2 = read_fixture_items(name2);
        let study_list2 = StudyList::new(name2, items2);
        controller.add(study_list2.clone()).unwrap();

        let result = controller.remove(name1);
        assert!(result.is_ok());

        let lists = controller.lists().unwrap();
        assert_eq!(lists.len(), 1);

        assert_eq!(controller.list(name2).unwrap(), study_list2);
        assert!(controller.list(name1).is_err());
    }

    #[test]
    fn test_invalid_remove() {
        let controller = setup();

        let err = controller.remove("invalid list").unwrap_err();
        assert!(matches!(err, ErrorKind::List(StudyListError::ListNotFound)));
    }

    #[test]
    fn test_valid_select() {
        let controller = setup();

        let name = "list1";
        let items = read_fixture_items(name);
        let study_list = StudyList::new(name, items);

        controller.add(study_list).unwrap();

        let result = controller.select(name);
        assert!(result.is_ok());

        let result = controller.selected_list().unwrap();
        assert_eq!(result.unwrap(), name);
    }

    #[test]
    fn test_invalid_select() {
        let controller = setup();

        let err = controller.select("invalid_list").unwrap_err();
        assert!(matches!(err, ErrorKind::List(StudyListError::ListNotFound)));

        let result = controller.selected_list();
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_list() {
        let controller = setup();

        let lists = controller.lists().unwrap();
        assert_eq!(lists.len(), 0);

        let list_name1 = "list1";
        let items1 = read_fixture_items(list_name1);
        let study_list1 = StudyList::new(list_name1, items1);
        controller.add(study_list1.clone()).unwrap();

        let result = controller.lists();
        assert!(result.is_ok());

        let lists = result.unwrap();
        assert!(lists.contains(&study_list1));
        assert_eq!(lists.len(), 1);

        let list_name2 = "list2";
        let items2 = read_fixture_items(list_name2);
        let study_list2 = StudyList::new(list_name2, items2);
        controller.add(study_list2.clone()).unwrap();

        let result = controller.lists();
        assert!(result.is_ok());

        let lists = result.unwrap();
        assert!(lists.contains(&study_list1));
        assert!(lists.contains(&study_list2));
        assert_eq!(lists.len(), 2);
    }
}
