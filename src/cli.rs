use std::fs;

use anyhow::{Context, Error, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;

use crate::commands::{DailyArgs, DictionaryArgs, DiscoveryArgs, StudyArgs};
use crate::config::{app_cache_dir, app_config_dir, app_data_dir};
use crate::context::GlobalContext;
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

    let dirs = ProjectDirs::from("", "", "nika").with_context(|| "failed to get project dirs")?;
    let ctx = GlobalContext::new(dirs).with_context(|| "failed to create global context")?;

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
