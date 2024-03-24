use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::cli::messages::EMPTY_STUDY_LISTS;
use crate::core::controllers::study_controller::StudyController;

#[derive(Args)]
pub struct ListArgs {}

impl StudyCommandHandler for ListArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        // TODO: show more details for each list:
        //       - current/total (perc %)
        //       - items per day
        //       - last study session

        let lists = controller.lists()?;

        if lists.is_empty() {
            println!("{}", EMPTY_STUDY_LISTS);
            return Ok(());
        }

        for (i, item) in lists.iter().enumerate() {
            let mut fmt_item = format!("{}. {}", i, item.name);

            if controller.selected_list()?.is_some_and(|c| c == item.name) {
                fmt_item = format!("{} (selected)", fmt_item);
            }

            println!("{}", fmt_item);
        }

        Ok(())
    }
}
