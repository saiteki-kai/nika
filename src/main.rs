mod api;
mod commands;
mod utils;

use clap::{Parser, Subcommand};
use commands::{CommandHandler, DailyArgs, ProgressArgs, SearchArgs};

#[derive(Parser)]
#[command(author, version, about, long_about = "<Long About>")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search words
    Search(SearchArgs),
    /// Show daily words
    Daily(DailyArgs),
    /// Show progress
    Progress(ProgressArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Search(args) => SearchArgs::handle(args),
        Command::Daily(args) => DailyArgs::handle(args),
        Command::Progress(args) => ProgressArgs::handle(args),
    }
}

#[test]
fn test_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
