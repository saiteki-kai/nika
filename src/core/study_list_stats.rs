use std::{fs, io, path::PathBuf, result};

use serde_json::Error as SerdeError;
use thiserror::Error;

use crate::core::models::study_list::{StudyConfig, StudyListConfig};

#[derive(Error, Debug)]
pub enum StudyListError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] SerdeError),
    #[error("List not found")]
    ListNotFound,
}

pub type Result<T> = result::Result<T, StudyListError>;

fn load(filepath: &PathBuf) -> Result<StudyListConfig> {
    if !filepath.exists() {
        let json = serde_json::to_string::<StudyListConfig>(&StudyListConfig::default())?;
        fs::write(filepath, json)?;
    }

    let file = fs::read_to_string(filepath)?;
    let config = serde_json::from_str::<StudyListConfig>(&file)?;

    Ok(config)
}

fn save(filepath: &PathBuf, config: &StudyListConfig) -> Result<()> {
    let json = serde_json::to_string::<StudyListConfig>(config)?;
    fs::write(filepath, json)?;

    Ok(())
}

pub struct StudyListStats {
    filepath: PathBuf,
    config: StudyListConfig,
}

impl StudyListStats {
    pub fn new(filepath: PathBuf) -> Result<Self> {
        let config = load(&filepath)?;

        Ok(StudyListStats { filepath, config })
    }

    pub fn get_list(&self, list_name: &str) -> Option<&StudyConfig> {
        self.config.lists.get(list_name)
    }

    pub fn get_lists(&self) -> Vec<String> {
        self.config.lists.keys().cloned().collect()
    }

    pub fn select_list(&mut self, list_name: &str) -> Result<()> {
        if self.config.lists.contains_key(list_name) {
            self.config.current = Some(list_name.to_string());

            return save(&self.filepath, &self.config);
        }

        Err(StudyListError::ListNotFound)
    }

    pub fn update_stats(&mut self, list_name: &str, config: StudyConfig) -> Result<()> {
        self.config.lists.insert(list_name.to_string(), config);

        save(&self.filepath, &self.config)
    }

    pub fn remove_stats(&mut self, list_name: &str) -> Result<()> {
        let result = self.config.lists.remove(list_name);

        if result.is_none() {
            return Err(StudyListError::ListNotFound);
        }

        if let Some(current) = &self.config.current {
            if current == list_name {
                self.config.current = None;
            }
        }

        save(&self.filepath, &self.config)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs, path::PathBuf};

    use tempfile::TempDir;
    use test_case::test_case;

    use crate::core::{
        models::study_list::{StudyConfig, StudyListConfig},
        study_list_stats::{load, save, StudyListError},
    };

    use super::StudyListStats;

    fn setup() -> (PathBuf, StudyListStats) {
        let tmp_dir: TempDir = TempDir::new().unwrap();
        let tmp_path = tmp_dir.into_path();

        let filepath = tmp_path.join("stats.json");
        let study_list_stats = StudyListStats::new(filepath.clone()).unwrap();

        (tmp_path, study_list_stats)
    }

    #[test]
    fn test_creation() {
        let (_, study_list_stats) = setup();

        assert!(study_list_stats.filepath.exists());

        assert!(study_list_stats.config.current.is_none());
        assert_eq!(study_list_stats.config.lists.len(), 0);
    }

    #[test]
    fn test_invalid_list() {
        let (_, mut study_list_stats) = setup();

        assert!(study_list_stats.get_list("invalid_list").is_none());
        assert!(study_list_stats.select_list("invalid_list").is_err());
        assert!(study_list_stats.remove_stats("invalid_list").is_err());

        assert!(study_list_stats.config.current.is_none());
        assert_eq!(study_list_stats.config.lists.len(), 0);
    }

    #[test]
    fn test_select_list() {
        let (_, mut study_list_stats) = setup();

        let config = StudyConfig::default();
        study_list_stats.update_stats("valid_name", config).unwrap();

        assert!(study_list_stats.select_list("valid_name").is_ok());
        assert_eq!(study_list_stats.config.current.unwrap(), "valid_name");
    }

    #[test_case("list_1", StudyConfig::default())]
    #[test_case("list_2", StudyConfig { current_index: 0, items_per_day: 0 })]
    #[test_case("list_3", StudyConfig { current_index: 42, items_per_day: 5 })]
    fn test_get_list_valid(name: &str, config: StudyConfig) {
        let (_, mut study_list_stats) = setup();

        study_list_stats.update_stats(name, config).unwrap();
        assert_eq!(study_list_stats.get_list(name).unwrap(), &config);
    }

    #[test_case(vec![])]
    #[test_case(vec!["list_1"])]
    #[test_case(vec!["list_1", "list_2"])]
    #[test_case(vec!["list_1", "list_2", "list_3"])]
    fn test_get_lists(items: Vec<&str>) {
        let (_, mut study_list_stats) = setup();

        let config = StudyConfig::default();

        for item in &items {
            study_list_stats.update_stats(item, config).unwrap();
        }

        let lists = study_list_stats.get_lists();

        assert_eq!(
            lists
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
            items
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>()
        );
        assert_eq!(lists.len(), items.len());
        assert_eq!(study_list_stats.config.lists.len(), items.len());
    }

    #[test]
    fn test_remove_list() {
        let (_, mut study_list_stats) = setup();

        let config = StudyConfig::default();
        study_list_stats.update_stats("list_1", config).unwrap();
        study_list_stats.update_stats("list_2", config).unwrap();
        study_list_stats.update_stats("list_3", config).unwrap();

        assert_eq!(study_list_stats.get_lists().len(), 3);

        study_list_stats.remove_stats("list_2").unwrap();

        assert!(study_list_stats.get_list("list_1").is_some());
        assert!(study_list_stats.get_list("list_2").is_none());
        assert!(study_list_stats.get_list("list_3").is_some());

        assert_eq!(study_list_stats.get_lists().len(), 2);
    }

    #[test]
    fn test_remove_selected_list() {
        let (_, mut study_list_stats) = setup();

        let config = StudyConfig::default();
        study_list_stats.update_stats("list_1", config).unwrap();
        study_list_stats.select_list("list_1").unwrap();

        assert!(study_list_stats.get_list("list_1").is_some());
        assert!(study_list_stats.select_list("list_1").is_ok());
        assert_eq!(study_list_stats.config.current.clone().unwrap(), "list_1");

        study_list_stats.remove_stats("list_1").unwrap();

        assert!(study_list_stats.get_list("list_1").is_none());
        assert!(study_list_stats.select_list("list_1").is_err());
        assert!(study_list_stats.config.current.is_none());
    }

    #[test]
    fn test_file_not_found() {
        let (tmp_dir, study_list_stats) = setup();
        fs::remove_dir_all(tmp_dir).unwrap();

        let filepath = &study_list_stats.filepath;
        let config = StudyListConfig::default();

        assert!(matches!(
            load(filepath).err().unwrap(),
            StudyListError::Io(_)
        ));

        assert!(matches!(
            save(filepath, &config).err().unwrap(),
            StudyListError::Io(_)
        ));
    }
}
