use anyhow::{Error, Result};
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct RemoveArgs {
    name: String,
}

impl StudyCommandHandler for RemoveArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        let result = manager.remove(&self.name);

        if result.is_ok() {
            println!("List '{}' has been removed", &self.name);
        } else {
            eprintln!("List '{}' not found", &self.name);
        }

        Ok(())
    }
}
