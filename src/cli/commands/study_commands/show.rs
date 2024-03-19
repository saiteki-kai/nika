use anyhow::Error;
use anyhow::Result;
use clap::Args;

use super::utils::get_list_name;
use crate::cli::handlers::StudyCommandHandler;
use crate::core::controllers::study_controller::StudyController;

#[derive(Args)]
pub struct ShowArgs {
    #[arg(short = 'n', long = "name")]
    name: Option<String>,
}

impl StudyCommandHandler for ShowArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        let list_name =
            get_list_name(self.name.as_deref(), controller.selected_list()?.as_deref())?;

        let words = controller.study_words(&list_name, false)?;

        // TODO: filter (by status: skipped, done, ...)

        for word in words {
            println!("{:?}", word);
        }

        Ok(())
    }
}
