use anyhow::Result;
use clap::{Parser, Subcommand};

use nika::cli::app::set_global_config;
use nika::cli::commands::{CommandHandler, DailyArgs, ProgressArgs, RandomArgs, SearchArgs};
use nika::cli::config::load_config;

#[derive(Parser)]
#[command(author, version, about, long_about = "<Long About>")]
struct Cli {
    #[command(subcommand)]
    commands: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search words
    Search(SearchArgs),
    /// Show daily words
    Daily(DailyArgs),
    /// Show progress
    Progress(ProgressArgs),
    /// Show a random word or kanji
    Random(RandomArgs),
}

async fn run() -> Result<()> {
    let config = load_config()?;
    set_global_config(config);

    let cli = Cli::parse();

    match &cli.commands {
        Command::Search(args) => SearchArgs::handle(args),
        Command::Daily(args) => DailyArgs::handle(args),
        Command::Progress(args) => ProgressArgs::handle(args),
        Command::Random(args) => RandomArgs::handle(args),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
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
