mod mark;
mod new;
mod progress;
mod review;

use anyhow::Ok;
use clap::{Args, Subcommand};

use self::new::{handle_new, NewArgs};
use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::handlers::CommandHandler;

#[derive(Subcommand)]
enum StudyCommand {
    /// Study new words from daily and discovery lists
    New(NewArgs),
    /// Review the words you have learned
    Review,
    /// Mark a word as learned manually
    Mark,
    /// Show your progress
    Progress,
}

#[derive(Args)]
pub struct StudyArgs {
    #[command(subcommand)]
    commands: StudyCommand,
}

impl CommandHandler for StudyArgs {
    fn handle(&self, ctx: &GlobalContext) -> CliResult<()> {
        match &self.commands {
            StudyCommand::New(args) => handle_new(ctx, args),
            StudyCommand::Review => Ok(println!("not implemented yet")),
            StudyCommand::Mark => Ok(println!("not implemented yet")),
            StudyCommand::Progress => Ok(println!("not implemented yet")),
        }
    }
}
