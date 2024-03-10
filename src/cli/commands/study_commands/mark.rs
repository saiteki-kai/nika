use anyhow::{Error, Result};
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct MarkArgs {
    pub word: String,
    pub status: String,
}

impl StudyCommandHandler for MarkArgs {
    fn handle(&self, _manager: &mut StudyListManager) -> Result<(), Error> {
        println!("{:?} marked as {:?}", self.word, self.status);
        Ok(())
    }
}
