use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use crate::context::Context;
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
    fn handle(&self, ctx: &Context) -> Result<(), anyhow::Error> {
        match &self.commands {
            DailyCommand::Import(args) => handle_import(ctx, args),
            DailyCommand::List(args) => handle_list(ctx, args),
        }
    }
}

#[derive(Args)]
struct ImportArgs {
    /// The file to import
    #[arg(required = true)]
    file: PathBuf,
}

fn handle_import(_ctx: &Context, _args: &ImportArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}

#[derive(Args)]
pub struct ListArgs {
    /// Show all the words in the daily list
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    all: Option<bool>,

    /// Limit the number of words to show
    #[arg(short, long)]
    count: Option<usize>,

    /// Show only the words in a specific status
    #[arg(short, long)]
    status: Option<String>,
}

fn handle_list(_ctx: &Context, _args: &ListArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}
