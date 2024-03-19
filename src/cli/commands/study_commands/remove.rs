use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::controllers::study_controller::StudyController;

#[derive(Args)]
pub struct RemoveArgs {
    name: String,
}

impl StudyCommandHandler for RemoveArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        let result = controller.remove(&self.name);

        if result.is_ok() {
            println!("List '{}' has been removed", &self.name);
        } else {
            eprintln!("List '{}' not found", &self.name);
        }

        Ok(())
    }
}
