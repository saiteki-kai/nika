use anyhow::{anyhow, Error, Result};

pub fn get_list_name(name: Option<String>, current: Option<String>) -> Result<String, Error> {
    let list_name: String = match &name {
        Some(name) => name.clone(),
        None => current.clone().ok_or_else(|| {
            anyhow!(
                "No list selected. Select the current study list using 'nika study select <name>'"
            )
        })?,
    };

    Ok(list_name)
}
