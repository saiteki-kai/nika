use anyhow::Error;
use anyhow::Result;
use clap::Args;
use nika_core::controllers::study_controller::StudyController;

use crate::handlers::StudyCommandHandler;

#[derive(Args)]
pub struct RemoveArgs {
    name: String,
}

impl StudyCommandHandler for RemoveArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        let result = controller.remove(&self.name);

        if result.is_ok() {
            println!("List '{}' removed", &self.name);
        } else {
            eprintln!("List '{}' not found", &self.name);
        }

        Ok(())
    }
}
