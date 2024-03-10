use anyhow::{Error, Result};
use clap::{Parser, Subcommand};

use nika::cli::app::{init_config, init_dictionary, init_folders};
use nika::cli::commands::{ProgressArgs, RandomArgs, SearchArgs, StudyArgs};
use nika::cli::handlers::CommandHandler;

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
    ///
    Study(StudyArgs),
    /// Show progress
    Progress(ProgressArgs),
    /// Show a random word or kanji
    Random(RandomArgs),
}

fn run() -> Result<(), Error> {
    init_config()?;

    init_folders()?;
    init_dictionary();

    let cli = Cli::parse();

    match &cli.commands {
        Command::Search(args) => args.handle(),
        Command::Study(args) => args.handle(),
        Command::Progress(args) => args.handle(),
        Command::Random(args) => args.handle(),
    }
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
