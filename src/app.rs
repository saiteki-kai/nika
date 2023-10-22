use crate::config::UserConfig;
use std::sync::OnceLock;

static CONFIG: OnceLock<UserConfig> = OnceLock::new();

pub fn set_global_config(config: UserConfig) {
    CONFIG.set(config).expect("could not set config")
}

pub fn get_global_config() -> &'static UserConfig {
    CONFIG.get().expect("config is not initialized")
}
