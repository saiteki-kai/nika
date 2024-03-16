use anyhow::Error;
use anyhow::Result;
use clap::Args;
use clap::Subcommand;

use super::study_commands::AddArgs;
use super::study_commands::DailyArgs;
use super::study_commands::ListArgs;
use super::study_commands::MarkArgs;
use super::study_commands::RemoveArgs;
use super::study_commands::SelectArgs;
use super::study_commands::SetArgs;
use crate::cli::handlers::CommandHandler;
use crate::cli::handlers::StudyCommandHandler;
use crate::config::app_data_dir;
use crate::config::STUDY_STATS_PATH;
use crate::core::study_list_manager::StudyListManager;

#[derive(Subcommand)]
pub enum StudyCommands {
    List(ListArgs),
    Select(SelectArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    Daily(DailyArgs),
    Mark(MarkArgs),
    Set(SetArgs),
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
            StudyCommands::Set(args) => args.handle(&mut study_list_manager),
        }
    }
}
