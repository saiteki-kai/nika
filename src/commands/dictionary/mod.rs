mod random;
mod search;

use clap::{Args, Subcommand};

use self::random::{handle_random, RandomArgs};
use self::search::{handle_search, SearchArgs};
use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::handlers::CommandHandler;

#[derive(Subcommand)]
enum DictionaryCommand {
    /// Search for a word in the dictionary
    Search(SearchArgs),
    /// Get a random word from the dictionary
    Random(RandomArgs),
}

#[derive(Args)]
pub struct DictionaryArgs {
    #[command(subcommand)]
    commands: DictionaryCommand,
}

impl CommandHandler for DictionaryArgs {
    fn handle(&self, ctx: &GlobalContext) -> CliResult<()> {
        match &self.commands {
            DictionaryCommand::Search(args) => handle_search(ctx, args),
            DictionaryCommand::Random(args) => handle_random(ctx, args),
        }
    }
}
