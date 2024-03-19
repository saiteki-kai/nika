use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

pub fn get_list_name(name: Option<&str>, current: Option<&str>) -> Result<String, Error> {
    let list_name: String = match name {
        Some(name) => name.to_string(),
        None => current
            .ok_or_else(|| {
                anyhow!(
                "No list selected. Select the current study list using 'nika study select <NAME>'"
            )
            })?
            .to_string(),
    };

    Ok(list_name)
}
