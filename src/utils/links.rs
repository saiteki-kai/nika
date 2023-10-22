use serde::{Deserialize, Serialize};

use crate::app::get_global_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub base_url: String,
}

pub fn generate_hyperlink(text: &str, url: &str) -> String {
    format!(
        "\u{1b}]8;id={};{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
        "", url, text
    )
}

pub fn get_links() -> Vec<Link> {
    let config = get_global_config();
    config.dictionaries.clone()
}
