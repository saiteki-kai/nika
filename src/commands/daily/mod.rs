mod import;
mod list;

use clap::{Args, Subcommand};

use self::import::{handle_import, ImportArgs};
use self::list::{handle_list, ListArgs};
use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::handlers::CommandHandler;

#[derive(Subcommand)]
enum DailyCommand {
    /// Import a list of words from a file
    Import(ImportArgs),
    /// List the words in the daily list
    List(ListArgs),
}

#[derive(Args)]
pub struct DailyArgs {
    #[command(subcommand)]
    commands: DailyCommand,
}

impl CommandHandler for DailyArgs {
    fn handle(&self, ctx: &GlobalContext) -> CliResult<()> {
        match &self.commands {
            DailyCommand::Import(args) => handle_import(ctx, args),
            DailyCommand::List(args) => handle_list(ctx, args),
        }
    }
}
