use anyhow::{Error, Result};
use clap::{Args, Subcommand};

use crate::cli::handlers::{CommandHandler, StudyCommandHandler};
use crate::config::{app_data_dir, STUDY_STATS_PATH};
use crate::core::study_list_manager::StudyListManager;

use super::study_commands::{AddArgs, DailyArgs, ListArgs, MarkArgs, RemoveArgs, SelectArgs};

#[derive(Subcommand)]
pub enum StudyCommands {
    List(ListArgs),
    Select(SelectArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    Daily(DailyArgs),
    Mark(MarkArgs),
}

#[derive(Args)]
pub struct StudyArgs {
    #[command(subcommand)]
    commands: StudyCommands,
}

impl CommandHandler for StudyArgs {
    fn handle(&self) -> Result<(), Error> {
        let mut study_list_manager =
            StudyListManager::new(app_data_dir(), STUDY_STATS_PATH.to_path_buf())?;

        match &self.commands {
            StudyCommands::Mark(args) => args.handle(&mut study_list_manager),
            StudyCommands::Daily(args) => args.handle(&mut study_list_manager),
            StudyCommands::Add(args) => args.handle(&mut study_list_manager),
            StudyCommands::Select(args) => args.handle(&mut study_list_manager),
            StudyCommands::Remove(args) => args.handle(&mut study_list_manager),
            StudyCommands::List(args) => args.handle(&mut study_list_manager),
        }
    }
}
