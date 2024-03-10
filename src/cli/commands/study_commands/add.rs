use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct AddArgs {
    name: String,
    file: PathBuf,
}

impl StudyCommandHandler for AddArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        let is_empty = manager.list().is_empty();

        manager.add(&self.name, &self.file)?;

        if is_empty {
            manager.select(&self.name)?;
        }

        Ok(())
    }
}
