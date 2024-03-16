use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct SelectArgs {
    name: String,
}

impl StudyCommandHandler for SelectArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        manager.select(&self.name)?;
        Ok(())
    }
}
