use anyhow::{Error, Result};
use std::{fs, path::PathBuf};

use crate::core::models::study_list::StudyConfig;

use super::study_list_stats::StudyListStats;

pub struct StudyListManager {
    dirpath: PathBuf,
    stats: StudyListStats,
}

impl StudyListManager {
    pub fn new(dirpath: PathBuf, stats_file: PathBuf) -> Self {
        StudyListManager {
            dirpath,
            stats: StudyListStats::new(stats_file),
        }
    }

    pub fn add(&mut self, name: &str, filepath: &PathBuf) -> Result<(), Error> {
        let new_filepath = self.dirpath.join(name);
        fs::copy(filepath, new_filepath)?;

        self.stats.update_stats(name, StudyConfig::default())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), Error> {
        let filepath = self.dirpath.join(name);
        fs::remove_file(filepath)?;

        self.stats.remove_stats(name)
    }

    pub fn select_study_list(&mut self, name: &str) -> Result<(), Error> {
        self.stats.select_list(name)
    }

    pub fn get(&self, name: &str) -> Option<&StudyConfig> {
        self.stats.get_list(name)
    }

    pub fn list(&self) -> Vec<String> {
        self.stats.get_lists()
    }
}
