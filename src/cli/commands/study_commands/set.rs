use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::models::study_list_config::StudyConfig;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct SetArgs {
    name: String,
    #[arg(short = 'c', long = "count")]
    items_per_day: Option<usize>,
}

impl StudyCommandHandler for SetArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        let list_config = manager.get(&self.name).with_context(|| "List not found")?;

        if let Some(count) = self.items_per_day {
            let config = StudyConfig {
                items_per_day: count,
                ..*list_config
            };

            manager.set(&self.name, config)?;
        }

        Ok(())
    }
}
