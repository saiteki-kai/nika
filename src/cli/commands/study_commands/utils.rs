use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use crate::cli::messages::NO_LIST_SELECTED;

pub fn get_list_name(name: Option<&str>, current: Option<&str>) -> Result<String, Error> {
    let list_name: String = match name {
        Some(name) => name.to_string(),
        None => current
            .ok_or_else(|| anyhow!(NO_LIST_SELECTED))?
            .to_string(),
    };

    Ok(list_name)
}
