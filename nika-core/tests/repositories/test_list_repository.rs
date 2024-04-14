use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use nika_core::errors::NikaError;
use nika_core::errors::StudyListError;
use nika_core::models::study_list::StudyConfig;
use nika_core::models::study_list::StudyList;
use nika_core::repositories::list_repository::ListRepository;
use tempfile::tempdir;
use tempfile::TempDir;
use test_case::test_case;

fn setup() -> (PathBuf, ListRepository) {
    let tmp_dir: TempDir = tempdir().expect("Failed to create temporary directory");
    let tmp_path = tmp_dir.into_path();

    let list_repo = ListRepository::new(tmp_path.clone());

    (tmp_path, list_repo)
}

#[test]
fn test_initialization() {
    let (_, list_repo) = setup();

    let lists = list_repo.get_lists().unwrap();
    assert!(lists.is_empty());
}

#[test]
fn test_invalid_list_name() {
    let (_, list_repo) = setup();

    assert!(list_repo.get_list("invalid_list").is_err());
    assert!(list_repo
        .update_list_config("invalid_list", StudyConfig::default())
        .is_err());
    assert!(list_repo.remove_list("invalid_list").is_err());

    assert_eq!(list_repo.get_lists().unwrap().len(), 0);
}

#[test_case("list_1", StudyConfig::default())]
#[test_case("list_2", StudyConfig { current_index: 0, items_per_day: 0 })]
#[test_case("list_3", StudyConfig { current_index: 42, items_per_day: 5 })]
fn test_get_list(name: &str, config: StudyConfig) {
    let (tmp_path, list_repo) = setup();

    let mut study_list = StudyList::new(name, vec![]);
    study_list.config = config.clone();

    let result = list_repo.add_list(study_list);
    assert!(result.is_ok());

    let result = list_repo.get_list(name);
    assert!(result.is_ok());

    let study_list = result.unwrap();
    assert_eq!(study_list.name, name);
    assert_eq!(study_list.config, config);

    assert!(tmp_path.join(format!("{}.bin", name)).exists());
}

#[test_case(vec![])]
#[test_case(vec!["list_1"])]
#[test_case(vec!["list_1", "list_2"])]
#[test_case(vec!["list_1", "list_2", "list_3"])]
fn test_get_lists(items: Vec<&str>) {
    let (_, list_repo) = setup();

    for item in &items {
        let study_list = StudyList::new(item, vec![]);
        list_repo.add_list(study_list).unwrap();
    }

    let lists = list_repo.get_lists().unwrap();

    assert_eq!(
        lists
            .iter()
            .map(|s| s.name.to_string())
            .collect::<HashSet<String>>(),
        items
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>()
    );
    assert_eq!(lists.len(), items.len());
}

#[test]
fn test_remove_list() {
    let (tmp_path, list_repo) = setup();

    list_repo
        .add_list(StudyList::new("list_1", vec![]))
        .unwrap();
    list_repo
        .add_list(StudyList::new("list_2", vec![]))
        .unwrap();
    list_repo
        .add_list(StudyList::new("list_3", vec![]))
        .unwrap();

    let result = list_repo.remove_list("list_2");
    assert!(result.is_ok());

    assert!(list_repo.get_list("list_1").is_ok());
    assert!(list_repo.get_list("list_3").is_ok());

    let result = list_repo.get_list("list_2");
    assert!(result.is_err());

    assert!(matches!(
        result.unwrap_err(),
        NikaError::List(StudyListError::ListNotFound)
    ));

    let lists = list_repo.get_lists().unwrap();
    assert_eq!(lists.len(), 2);

    assert!(tmp_path.join("list_1.bin").exists());
    assert!(!tmp_path.join("list_2.bin").exists());
    assert!(tmp_path.join("list_3.bin").exists());
}

#[test]
fn test_file_not_found() {
    let (tmp_dir, list_repo) = setup();
    fs::remove_dir_all(tmp_dir).unwrap();

    assert!(matches!(
        list_repo.get_lists().unwrap_err(),
        NikaError::Io(_),
    ));
}
