use std::{env, fs, path::PathBuf};

use tempfile::tempdir;

use nika::core::{
    models::study_list::{StudyConfig, StudyListConfig},
    study_list_manager::StudyListManager,
    study_list_stats::StudyListError,
};

fn setup() -> (PathBuf, PathBuf, StudyListManager) {
    let tmp_dir = tempdir().unwrap();
    let tmp_path = tmp_dir.into_path();

    let filepath = tmp_path.join("stats.json");
    let study_list_manager = StudyListManager::new(tmp_path.clone(), filepath.clone()).unwrap();

    (tmp_path, filepath, study_list_manager)
}

fn get_fixture(name: &str) -> PathBuf {
    if name != "list1" && name != "list2" {
        eprintln!("The only lists availables are 'list1' and 'list2'");
    }

    let fixtures_path = env::current_dir()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join("study_list");

    fixtures_path.join(name)
}

#[test]
fn test_initialization() {
    let (_tmp_path, stats_filepath, study_list_manager) = setup();

    assert!(stats_filepath.exists());
    assert_eq!(study_list_manager.list().len(), 0);
}

#[test]
fn test_valid_add() {
    let (tmp_path, stats_filepath, mut study_list_manager) = setup();

    let name = "list1";
    let list1 = get_fixture(name);

    let result = study_list_manager.add(name, &list1);

    assert!(result.is_ok());

    assert!(tmp_path.join(name).exists());

    let list_config = StudyListConfig::load(&stats_filepath).unwrap();
    assert!(list_config.current.is_none());
    assert!(list_config.lists.contains_key(name));
    assert_eq!(list_config.lists.len(), 1);
    assert_eq!(list_config.lists[name], StudyConfig::default());
}

#[test]
fn test_duplicated_add() {
    let (tmp_path, stats_filepath, mut study_list_manager) = setup();

    let name1 = "list1";
    let list1 = get_fixture(name1);

    let name2 = "list2";
    let list2 = get_fixture(name2);

    let result1 = study_list_manager.add(name1, &list1);
    let result2 = study_list_manager.add(name1, &list2);

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    assert!(tmp_path.join(name1).exists());

    let a = fs::read_to_string(tmp_path.join(name1)).unwrap();
    let b = fs::read_to_string(list2).unwrap();
    assert_eq!(a, b);

    let list_config = StudyListConfig::load(&stats_filepath).unwrap();
    assert!(list_config.lists.contains_key(name1));
    assert!(!list_config.lists.contains_key(name2));
    assert_eq!(list_config.lists.len(), 1);
}

#[test]
fn test_invalid_add() {
    let (tmp_path, _stats_filepath, mut study_list_manager) = setup();

    let list_name = "list";
    let invalid_filepath = PathBuf::from("invalid_list");

    let result = study_list_manager.add(list_name, &invalid_filepath);

    assert!(matches!(result.unwrap_err(), StudyListError::Io(_)));

    assert!(!invalid_filepath.exists());
    assert!(!tmp_path.join(list_name).exists());
}

#[test]
fn test_remove() {
    let (tmp_path, stats_filepath, mut study_list_manager) = setup();

    let list_name1 = "list1";
    let list1 = get_fixture(list_name1);
    study_list_manager.add(list_name1, &list1).unwrap();

    let list_name2 = "list2";
    let list2 = get_fixture(list_name2);
    study_list_manager.add(list_name2, &list2).unwrap();

    let result = study_list_manager.remove(list_name1);

    assert!(result.is_ok());

    assert!(!tmp_path.join(list_name1).exists());

    let list_config = StudyListConfig::load(&stats_filepath).unwrap();
    let lists = list_config.lists;
    assert!(!lists.contains_key(list_name1));
    assert!(lists.contains_key(list_name2));
    assert_eq!(lists.len(), 1);
}

#[test]
fn test_invalid_remove() {
    let (_, _, mut study_list_manager) = setup();

    let result = study_list_manager.remove("invalid list");

    assert!(matches!(result.unwrap_err(), StudyListError::Io(_)));
}

#[test]
fn test_valid_select() {
    let (_, stats_filepath, mut study_list_manager) = setup();

    let name = "list1";
    let list1 = get_fixture(name);
    study_list_manager.add(name, &list1).unwrap();

    let result = study_list_manager.select(name);

    assert!(result.is_ok());

    let list_config = StudyListConfig::load(&stats_filepath).unwrap();
    assert!(list_config.current.is_some());
    assert_eq!(list_config.current.unwrap(), name);
}

#[test]
fn test_invalid_select() {
    let (_, stats_filepath, mut study_list_manager) = setup();

    let result = study_list_manager.select("invalid_list");

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), StudyListError::ListNotFound));

    let list_config = StudyListConfig::load(&stats_filepath).unwrap();
    assert!(list_config.current.is_none());
}

#[test]
fn test_remove_select() {
    let (_, stats_filepath, mut study_list_manager) = setup();

    let name = "list1";
    let list1 = get_fixture(name);
    study_list_manager.add(name, &list1).unwrap();
    study_list_manager.select(name).unwrap();

    let result = study_list_manager.remove(name);

    assert!(result.is_ok());

    let list_config = StudyListConfig::load(&stats_filepath).unwrap();
    assert!(list_config.current.is_none());
}
