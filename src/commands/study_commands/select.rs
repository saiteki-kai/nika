use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use clap::Args;
use nika_core::controllers::study_controller::StudyController;

use crate::handlers::StudyCommandHandler;

#[derive(Args)]
pub struct SelectArgs {
    name: String,
}

impl StudyCommandHandler for SelectArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        controller.select(&self.name)?;
        println!("List '{}' selected", &self.name);
        Ok(())
    }
}
