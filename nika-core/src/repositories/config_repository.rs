use std::path::PathBuf;

use crate::errors::Result;
use crate::models::link::Link;
use crate::models::user_config::UserConfig;

pub struct ConfigRepository {
    path: PathBuf,
}

impl ConfigRepository {
    pub fn new(filepath: PathBuf) -> Self {
        Self { path: filepath }
    }

    pub fn dictionaries(&self) -> Result<Vec<Link>> {
        let config = self.load_config()?;
        Ok(config.dictionaries.clone())
    }

    pub fn set_current_list(&self, name: &str) -> Result<()> {
        let mut config = self.load_config()?;
        config.current_list = Some(name.to_string());
        self.save_config(config)
    }

    pub fn get_current_list(&self) -> Result<Option<String>> {
        let config = self.load_config()?;
        Ok(config.current_list)
    }

    fn load_config(&self) -> Result<UserConfig> {
        UserConfig::load(&self.path)
    }

    fn save_config(&self, config: UserConfig) -> Result<()> {
        UserConfig::save(&self.path, config)
    }
}
