use anyhow::Error;
use anyhow::Result;
use clap::Args;
use nika_core::controllers::study_controller::StudyController;
use nika_core::models::study_list::StudyConfig;

use crate::handlers::StudyCommandHandler;

#[derive(Args)]
pub struct SetArgs {
    name: String,
    #[arg(short = 'c', long = "count")]
    items_per_day: Option<usize>,
}

impl StudyCommandHandler for SetArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        let list = controller.list(&self.name)?;

        if let Some(count) = self.items_per_day {
            let config = StudyConfig {
                items_per_day: count,
                ..list.config
            };

            controller.update_config(&self.name, config)?;
        }

        Ok(())
    }
}
