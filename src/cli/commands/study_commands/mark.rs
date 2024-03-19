use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::controllers::study_controller::StudyController;

#[derive(Args)]
pub struct MarkArgs {
    pub word: String,
    pub status: String,
}

impl StudyCommandHandler for MarkArgs {
    fn handle(&self, _controller: &StudyController) -> Result<(), Error> {
        println!("{:?} marked as {:?}", self.word, self.status);
        Ok(())
    }
}
