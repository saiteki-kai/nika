use std::fs;

use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

use crate::commands::daily::DailyArgs;
use crate::commands::dictionary::DictionaryArgs;
use crate::commands::discovery::DiscoveryArgs;
use crate::commands::study::StudyArgs;
use crate::config::app_cache_dir;
use crate::config::app_config_dir;
use crate::config::app_data_dir;
use crate::context::Context;
use crate::handlers::CommandHandler;
use crate::utils::style::STYLES;

#[derive(Parser)]
#[command(author, version, about=None, styles=STYLES, disable_help_subcommand=true)]
struct Cli {
    #[command(subcommand)]
    commands: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Study, review or show progress
    Study(StudyArgs),
    /// List or import daily words
    Daily(DailyArgs),
    /// List, import, add or remove new discovered words
    Discovery(DiscoveryArgs),
    /// Search words in the dictionary
    Dictionary(DictionaryArgs),
}

pub fn run() -> Result<(), Error> {
    init_folders()?;

    let ctx = Context {
        // to be defined (preferences path, data path, database path, etc.)
    };

    let cli = Cli::parse();

    match &cli.commands {
        Command::Study(args) => args.handle(&ctx),
        Command::Daily(args) => args.handle(&ctx),
        Command::Discovery(args) => args.handle(&ctx),
        Command::Dictionary(args) => args.handle(&ctx),
    }
}

fn init_folders() -> Result<(), Error> {
    fs::create_dir_all(app_cache_dir())?;
    fs::create_dir_all(app_config_dir())?;
    fs::create_dir_all(app_data_dir())?;

    fs::create_dir_all(app_cache_dir().join("data"))?;
    fs::create_dir_all(app_data_dir().join("lists"))?;

    Ok(())
}

#[test]
fn test_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
