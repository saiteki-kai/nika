use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use crate::context::GlobalContext;
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
    fn handle(&self, ctx: &mut GlobalContext) -> Result<(), anyhow::Error> {
        match &self.commands {
            DiscoveryCommand::List(args) => handle_list(ctx, args),
            DiscoveryCommand::Add(args) => handle_add(ctx, args),
            DiscoveryCommand::Import(args) => handle_import(ctx, args),
            DiscoveryCommand::Remove(args) => handle_remove(ctx, args),
        }
    }
}

#[derive(Args)]
struct ImportArgs {
    /// The file to import
    #[arg(required = true)]
    file: PathBuf,
}

fn handle_import(_ctx: &mut GlobalContext, _args: &ImportArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}

#[derive(Args)]
struct ListArgs {
    /// Show all the words in the discovery list
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    all: Option<bool>,

    /// Limit the number of words to show
    #[arg(short, long)]
    count: Option<usize>,

    /// Show only the words in a specific status
    #[arg(short, long)]
    status: Option<String>,
}

fn handle_list(_ctx: &mut GlobalContext, _args: &ListArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}

#[derive(Args)]
struct AddArgs {
    /// Kanji or kana of the word
    text: Option<String>,
    /// Reading of the word
    reading: Option<String>,
    /// Meaning of the word
    meaning: Option<String>,
}

fn handle_add(_ctx: &mut GlobalContext, _args: &AddArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}

#[derive(Args)]
struct RemoveArgs {
    /// Kanji or kana of the word
    text: Option<String>,
    /// Reading of the word
    reading: Option<String>,
    /// Meaning of the word
    meaning: Option<String>,
}

fn handle_remove(_ctx: &mut GlobalContext, _args: &RemoveArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}
