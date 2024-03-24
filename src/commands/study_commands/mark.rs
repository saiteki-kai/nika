use anyhow::Error;
use anyhow::Result;
use clap::Args;
use nika_core::controllers::study_controller::StudyController;

use crate::handlers::StudyCommandHandler;

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
