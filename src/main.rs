use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = "<Long About>")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search words
    Search(commands::search::SearchArgs),
    /// Show daily words
    Daily(commands::daily::DailyArgs),
    /// Show progress
    Progress(commands::progress::ProgressArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Search(args) => commands::search::search(args),
        Command::Daily(args) => commands::daily::daily(args),
        Command::Progress(args) => commands::progress::progress(args),
    }
}

#[test]
fn test_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
