mod add;
mod import;
mod list;
mod remove;

use clap::{Args, Subcommand};

use self::add::{handle_add, AddArgs};
use self::import::{handle_import, ImportArgs};
use self::list::{handle_list, ListArgs};
use self::remove::{handle_remove, RemoveArgs};
use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::handlers::CommandHandler;

#[derive(Subcommand)]
enum DiscoveryCommand {
    /// Add a word to the discovery list
    Add(AddArgs),
    /// List the words in the discovery list
    List(ListArgs),
    /// Remove a word from the discovery list
    Remove(RemoveArgs),
    /// Add a list of words to the discovery list
    Import(ImportArgs),
}

#[derive(Args)]
pub struct DiscoveryArgs {
    #[command(subcommand)]
    commands: DiscoveryCommand,
}

impl CommandHandler for DiscoveryArgs {
    fn handle(&self, ctx: &GlobalContext) -> CliResult<()> {
        match &self.commands {
            DiscoveryCommand::List(args) => handle_list(ctx, args),
            DiscoveryCommand::Add(args) => handle_add(ctx, args),
            DiscoveryCommand::Import(args) => handle_import(ctx, args),
            DiscoveryCommand::Remove(args) => handle_remove(ctx, args),
        }
    }
}
