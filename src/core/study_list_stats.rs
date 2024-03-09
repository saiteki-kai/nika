use std::{fs, path::PathBuf};

use anyhow::{Error, Ok, Result};

use crate::core::models::study_list::{StudyConfig, StudyListConfig};

pub struct StudyListStats {
    filepath: PathBuf,
    config: StudyListConfig,
}

impl StudyListStats {
    pub fn new(filepath: PathBuf) -> Self {
        let config = StudyListStats::load(&filepath).unwrap_or_default();

        StudyListStats { filepath, config }
    }

    pub fn get_list(&self, list_name: &str) -> Option<&StudyConfig> {
        self.config.lists.get(list_name)
    }

    pub fn get_lists(&self) -> Vec<String> {
        self.config.lists.keys().cloned().collect()
    }

    pub fn select_list(&mut self, list_name: &str) -> Result<(), Error> {
        if self.config.lists.contains_key(list_name) {
            self.config.current = Some(list_name.to_string());
            return self.save();
        }

        Err(Error::msg("list not found"))
    }

    pub fn update_stats(&mut self, list_name: &str, config: StudyConfig) -> Result<(), Error> {
        self.config.lists.insert(list_name.to_string(), config);
        self.save()
    }

    pub fn remove_stats(&mut self, list_name: &str) -> Result<(), Error> {
        if !self.config.lists.contains_key(list_name) {
            return Err(Error::msg("Not found"));
        }

        self.config.lists.remove(list_name);

        if let Some(current) = &self.config.current {
            println!("{} {}", current, list_name);
            if current == list_name {
                self.config.current = None;
            }
        }

        self.save()
    }

    fn load(filepath: &PathBuf) -> Result<StudyListConfig, Error> {
        if !&filepath.exists() {
            let json = serde_json::to_string::<StudyListConfig>(&StudyListConfig::default())?;
            fs::write(filepath, json)?;
        }

        let file = fs::read_to_string(filepath)?;
        let config = serde_json::from_str::<StudyListConfig>(&file)?;

        Ok(config)
    }

    fn save(&self) -> Result<(), Error> {
        let json = serde_json::to_string::<StudyListConfig>(&self.config)?;
        fs::write(&self.filepath, json)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, path::PathBuf};

    use tempfile::TempDir;
    use test_case::test_case;

    use crate::core::models::study_list::StudyConfig;

    use super::StudyListStats;

    fn setup() -> (PathBuf, StudyListStats) {
        let tmp_dir: TempDir = TempDir::new().unwrap();
        let filepath = tmp_dir.into_path().join("stats.json");

        let study_list_stats = StudyListStats::new(filepath.clone());

        (filepath, study_list_stats)
    }

    #[test]
    fn test_creation() {
        let (filepath, study_list_stats) = setup();

        assert!(&filepath.exists());

        assert_eq!(study_list_stats.config.current, None);
        assert_eq!(study_list_stats.config.lists.len(), 0);
    }

    #[test]
    fn test_invalid_list() {
        let (_filepath, mut study_list_stats) = setup();

        assert!(study_list_stats.get_list("invalid_list").is_none());
        assert!(study_list_stats.select_list("invalid_list").is_err());
        assert!(study_list_stats.remove_stats("invalid_list").is_err());

        assert_eq!(study_list_stats.config.current, None);
        assert_eq!(study_list_stats.config.lists.len(), 0);
    }

    #[test]
    fn test_select_list() {
        let (_filepath, mut study_list_stats) = setup();

        let config = StudyConfig::default();
        study_list_stats.update_stats("valid_name", config).unwrap();

        assert!(study_list_stats.select_list("valid_name").is_ok());
        assert_eq!(study_list_stats.config.current.unwrap(), "valid_name");
    }

    #[test_case("list_1", StudyConfig::default())]
    #[test_case("list_2", StudyConfig { current_index: 0, items_per_day: 0 })]
    #[test_case("list_3", StudyConfig { current_index: 42, items_per_day: 5 })]
    fn test_get_list_valid(name: &str, config: StudyConfig) {
        let (_filepath, mut study_list_stats) = setup();

        study_list_stats.update_stats(name, config).unwrap();
        assert_eq!(study_list_stats.get_list(name).unwrap(), &config);
    }

    #[test_case(vec![])]
    #[test_case(vec!["list_1"])]
    #[test_case(vec!["list_1", "list_2"])]
    #[test_case(vec!["list_1", "list_2", "list_3"])]
    fn test_get_lists(items: Vec<&str>) {
        let (_filepath, mut study_list_stats) = setup();

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
        let (_filepath, mut study_list_stats) = setup();

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
        let (_filepath, mut study_list_stats) = setup();

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
}
