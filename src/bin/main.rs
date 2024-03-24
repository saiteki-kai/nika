use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use nika::app::init_folders;
use nika::commands::ProgressArgs;
use nika::commands::RandomArgs;
use nika::commands::SearchArgs;
use nika::commands::StudyArgs;
use nika::handlers::CommandHandler;

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
    /// Study
    Study(StudyArgs),
    /// Show progress
    Progress(ProgressArgs),
    /// Show a random word or kanji
    Random(RandomArgs),
}

fn run() -> Result<(), Error> {
    init_folders()?;

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
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    std::process::exit(0);
}

#[test]
fn test_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
