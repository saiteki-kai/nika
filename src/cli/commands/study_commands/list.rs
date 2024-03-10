use anyhow::{Error, Result};
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct ListArgs {
    name: String,
}

impl StudyCommandHandler for ListArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        // TODO: print "(current)" near the selected list.

        for (i, item) in manager.list().iter().enumerate() {
            println!("{}. {}", i, item);
        }
        Ok(())
    }
}
