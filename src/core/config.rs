use anyhow::{Context, Error};
use std::path::PathBuf;

use super::models::user_config::UserConfig;

#[derive(Debug)]
pub struct ConfigService {
    pub app_name: String,
    pub config_name: Option<String>,
}

impl ConfigService {
    pub fn new(app_name: String, config_name: Option<String>) -> Self {
        Self {
            app_name,
            config_name,
        }
    }

    pub fn config_path(&self) -> Result<PathBuf, Error> {
        confy::get_configuration_file_path(&self.app_name, self.config_name.as_deref())
            .with_context(|| "unable to find the configuration")
    }

    pub fn load_config(&self) -> Result<UserConfig, Error> {
        confy::load(&self.app_name, self.config_name.as_deref())
            .with_context(|| "unable to load the configuration")
    }

    pub fn save_config(&self, config: UserConfig) -> Result<(), Error> {
        confy::store(&self.app_name, self.config_name.as_deref(), config)
            .with_context(|| "unable to save the configuration")
    }
}
