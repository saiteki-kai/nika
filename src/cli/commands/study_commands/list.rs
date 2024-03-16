use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct ListArgs {}

impl StudyCommandHandler for ListArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        // TODO: show more details for each list:
        //       - current/total (perc %)
        //       - items per day
        //       - last study session

        for (i, item) in manager.list().iter().enumerate() {
            let mut fmt_item = format!("{}. {}", i, item);

            if manager.current.clone().is_some_and(|c| c == *item) {
                fmt_item = format!("{} (selected)", fmt_item);
            }

            println!("{}", fmt_item);
        }
        Ok(())
    }
}
