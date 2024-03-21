use std::fs;
use std::path::Path;
use std::path::PathBuf;

use nika::core::controllers::study_controller::StudyController;
use nika::core::errors::ErrorKind;
use nika::core::errors::StudyListError;
use nika::core::models::jmdict::JMdict;
use nika::core::models::study_list::StudyList;
use nika::core::repositories::config_repository::ConfigRepository;
use nika::core::repositories::dictionary_repository::DictionaryRepository;
use nika::core::repositories::dictionary_repository::TagMap;
use nika::core::repositories::dictionary_repository::WordMap;
use nika::core::repositories::list_repository::ListRepository;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use tempfile::tempdir;

fn setup() -> StudyController {
    let tmp_dir = tempdir().expect("Failed to create temporary directory");

    let lists_path = tmp_dir.into_path();
    let config_filepath = tempfile::NamedTempFile::new().unwrap().path().to_path_buf();

    let words_path = Path::new("tests").join("fixtures").join("words.json");
    let words = fs::read_to_string(words_path).unwrap();
    let data: JMdict = serde_json::from_str(&words).unwrap();

    let words: WordMap = data
        .words
        .into_par_iter()
        .map(|word| (word.id.clone(), word))
        .collect();

    let tags: TagMap = data.tags;

    let list_repository = ListRepository::new(lists_path);
    let config_repository = ConfigRepository::new(config_filepath);
    let dictionary_repository = DictionaryRepository::from(words, tags);

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
