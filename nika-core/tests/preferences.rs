use std::fs;

use nika_core::preferences::UserPreferences;
use tempfile::tempdir;

#[test]
fn test_load_unexistent_file() {
    let tempdir = tempdir().expect("Failed to create temporary directory");
    let filepath = tempdir.path().join("unexistent.json");

    let result = UserPreferences::load(&filepath);
    assert!(result.is_ok());

    let preferences = result.expect("Failed to load preferences");

    assert_eq!(
        preferences.external_dictionaries,
        UserPreferences::default().external_dictionaries
    );
}

#[test]
fn test_load_empty_file() {
    let tempdir = tempdir().expect("Failed to create temporary directory");
    let filepath = tempdir.path().join("existing.json");

    fs::write(&filepath, "").expect("Failed to write temporary file");

    let result = UserPreferences::load(&filepath);
    assert!(result.is_err());
}

#[test]
fn test_save_file() {
    let tempdir = tempdir().expect("Failed to create temporary directory");
    let filepath = tempdir.path().join("new-preferences.json");

    let mut preferences = UserPreferences::new(&filepath);
    preferences.external_dictionaries = vec![];

    let result = preferences.save();
    assert!(result.is_ok());

    let contents = fs::read_to_string(&filepath).expect("Failed to read the preferences file");
    assert!(contents.contains("external_dictionaries"));
    assert!(!contents.contains("filepath"));
}

#[test]
fn test_save_without_load() {
    let preferences = UserPreferences::default();
    let result = preferences.save();

    assert!(result.is_err());
}

#[test]
fn test_load_save_and_reload() {
    let tempdir = tempdir().expect("Failed to create temporary directory");
    let filepath = tempdir.path().join("preferences.json");

    let mut preferences = UserPreferences::load(&filepath).expect("Failed to load preferences");
    preferences.external_dictionaries.remove(0);

    preferences.save().expect("Failed to save preferences");

    let new_preferences = UserPreferences::load(&filepath).expect("Failed to load new preferences");
    assert_ne!(new_preferences, UserPreferences::default());
}
