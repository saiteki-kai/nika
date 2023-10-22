mod api;
mod app;
mod commands;
mod config;
mod utils;

use app::set_global_config;
use clap::{Parser, Subcommand};

use commands::{CommandHandler, DailyArgs, ProgressArgs, SearchArgs};
use config::load_config;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_global_config(load_config()?);

    let cli = Cli::parse();

    match &cli.command {
        Command::Search(args) => SearchArgs::handle(args),
        Command::Daily(args) => DailyArgs::handle(args),
        Command::Progress(args) => ProgressArgs::handle(args),
    }

    Ok(())
}

#[test]
fn test_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
