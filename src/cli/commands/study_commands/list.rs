use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::controllers::study_controller::StudyController;

#[derive(Args)]
pub struct ListArgs {}

impl StudyCommandHandler for ListArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        // TODO: show more details for each list:
        //       - current/total (perc %)
        //       - items per day
        //       - last study session
        //
        //       - if the list is empty -> show a message (add a list using the command
        //         add)

        for (i, item) in controller.lists()?.iter().enumerate() {
            let mut fmt_item = format!("{}. {}", i, item.name);

            if controller.selected_list()?.is_some_and(|c| c == item.name) {
                fmt_item = format!("{} (selected)", fmt_item);
            }

            println!("{}", fmt_item);
        }

        Ok(())
    }
}
