use clap::{Args, Subcommand};

use super::study_commands::{AddArgs, DailyArgs, ListArgs, MarkArgs, RemoveArgs, SelectArgs};
use super::CommandHandler;

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
    fn handle(&self) {
        match &self.commands {
            StudyCommands::Mark(args) => args.handle(),
            StudyCommands::Daily(args) => args.handle(),
            _ => (),
        }
    }
}
