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
    use tempfile::TempDir;

    use crate::core::models::study_list::StudyConfig;

    use super::StudyListStats;

    #[test]
    fn test_creation() {
        let tmp_dir: TempDir = TempDir::new().unwrap();
        let filepath = tmp_dir.into_path().join("stats.json");

        let study_list_stats = StudyListStats::new(filepath.clone());
        assert!(&filepath.exists());

        assert_eq!(study_list_stats.config.current, None);
        assert_eq!(study_list_stats.config.lists.len(), 0);
    }

    #[test]
    fn test_select_invalid_list() {
        let tmp_dir: TempDir = TempDir::new().unwrap();
        let filepath = tmp_dir.into_path().join("stats.json");

        let mut study_list_stats = StudyListStats::new(filepath.clone());
        assert!(study_list_stats.select_list("invalid_list").is_err());
    }

    #[test]
    fn test_select_valid_list() {
        let tmp_dir: TempDir = TempDir::new().unwrap();
        let filepath = tmp_dir.into_path().join("stats.json");

        let mut study_list_stats = StudyListStats::new(filepath.clone());
        study_list_stats
            .update_stats("valid_name", StudyConfig::default())
            .unwrap();
        assert!(study_list_stats.select_list("valid_name").is_ok());

        assert_eq!(study_list_stats.config.current.unwrap(), "valid_name");
    }
}
