use std::fs;

use anyhow::Error;
use anyhow::Result;

use crate::config::app_cache_dir;
use crate::config::app_config_dir;
use crate::config::app_data_dir;

pub fn init_folders() -> Result<(), Error> {
    fs::create_dir_all(app_cache_dir())?;
    fs::create_dir_all(app_config_dir())?;
    fs::create_dir_all(app_data_dir())?;

    fs::create_dir_all(app_cache_dir().join("data"))?;
    fs::create_dir_all(app_data_dir().join("lists"))?;

    Ok(())
}
