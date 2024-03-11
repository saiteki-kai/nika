use anyhow::{Error, Result};
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::models::study_list::StudyConfig;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct SetArgs {
    name: String,
    #[arg(short = 'c', long = "count")]
    items_per_day: Option<usize>,
}

impl StudyCommandHandler for SetArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        if let (Some(count), Some(list_config)) = (self.items_per_day, manager.get(&self.name)) {
            let config = StudyConfig {
                items_per_day: count,
                ..*list_config
            };

            manager.set(&self.name, config)?;
        }

        Ok(())
    }
}
