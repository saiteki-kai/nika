#![deny(unsafe_code)]
mod api;
mod app;
mod commands;
mod config;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

use app::set_global_config;
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

fn run() -> Result<()> {
    let config = load_config()?;
    set_global_config(config);

    let cli = Cli::parse();

    match &cli.command {
        Command::Search(args) => SearchArgs::handle(args),
        Command::Daily(args) => DailyArgs::handle(args),
        Command::Progress(args) => ProgressArgs::handle(args),
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }

    std::process::exit(0);
}

#[test]
fn test_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
