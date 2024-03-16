use std::fs;
use std::path::PathBuf;

use super::study_list_stats::Result;
use super::study_list_stats::StudyListStats;
use crate::core::models::study_list_config::StudyConfig;

pub struct StudyListManager {
    dirpath: PathBuf,
    stats: StudyListStats,
    pub current: Option<String>,
}

impl StudyListManager {
    pub fn new(dirpath: PathBuf, stats_file: PathBuf) -> Result<Self> {
        let stats = StudyListStats::new(stats_file)?;
        let current = stats.get_selected_list();

        Ok(StudyListManager {
            dirpath,
            stats,
            current,
        })
    }

    fn list_filepath(&self, name: &str) -> PathBuf {
        self.dirpath.join(name)
    }

    pub fn add(&mut self, name: &str, filepath: &PathBuf) -> Result<()> {
        let new_filepath = self.list_filepath(name);
        fs::copy(filepath, new_filepath)?;

        self.stats.update_stats(name, StudyConfig::default())
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        let filepath = self.list_filepath(name);
        fs::remove_file(filepath)?;

        self.stats.remove_stats(name)
    }

    pub fn select(&mut self, name: &str) -> Result<()> {
        self.stats.select_list(name)?;
        self.current = Some(name.to_string());

        Ok(())
    }

    pub fn set(&mut self, name: &str, config: StudyConfig) -> Result<()> {
        self.stats.update_stats(name, config)
    }

    pub fn get(&self, name: &str) -> Result<&StudyConfig> {
        self.stats.get_list(name)
    }

    pub fn list(&self) -> Vec<String> {
        self.stats.get_lists()
    }

    pub fn study(&self, name: &str, daily: bool) -> Result<Vec<String>> {
        let config = self.stats.get_list(name)?;

        let reader = fs::read_to_string(self.list_filepath(name))?;
        let id_list = reader.lines();

        let items: Vec<String> = if !daily {
            id_list.map(String::from).collect()
        } else {
            id_list
                .skip(config.current_index)
                .take(config.items_per_day)
                .map(String::from)
                .collect()
        };

        Ok(items)
    }
}
