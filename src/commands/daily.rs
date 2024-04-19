use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use nika_core::models::study_list::DailyItem;
use nika_core::models::study_list::DailyList;

use crate::context::GlobalContext;
use crate::error::CliResult;
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
    fn handle(&self, ctx: &GlobalContext) -> CliResult<()> {
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

fn handle_import(ctx: &GlobalContext, args: &ImportArgs) -> CliResult<()> {
    let content = fs::read_to_string(args.file.as_path())?;

    let data: HashSet<String> = content.lines().map(|s| s.to_string()).collect();
    let items: Vec<DailyItem> = data
        .iter()
        .enumerate()
        .map(|(i, d)| DailyItem::new(d.to_owned(), i as i64))
        .collect();

    let list = DailyList::new(items);

    let db = ctx.db()?;

    let pbar = indicatif::ProgressBar::new(list.items.len() as u64);
    pbar.set_style(indicatif::ProgressStyle::default_spinner());
    pbar.set_message("importing words...");

    db.import_daily_list(list)?;

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

fn handle_list(ctx: &GlobalContext, args: &ListArgs) -> CliResult<()> {
    let count = args.count.unwrap_or(0);

    let list = ctx
        .db()?
        .get_daily_list()
        .with_context(|| "failed to get study list")?;

    let mut items: Vec<DailyItem> = if count > 0 {
        list.items.iter().take(count).cloned().collect()
    } else {
        list.items
    };

    // TODO: use the enum directly (currently an invalid value will be converted to
    // "new")
    if let Some(status) = &args.status {
        items = items
            .iter()
            .filter(|i| i.progress.status == status.as_str().into())
            .cloned()
            .collect();
    }

    for item in items {
        println!("{:?}", item);
    }

    Ok(())
}
