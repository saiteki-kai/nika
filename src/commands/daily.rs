use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use nika_core::models::study_list::StudyItem;
use nika_core::models::study_list::StudyList;

use crate::context::GlobalContext;
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
    fn handle(&self, ctx: &mut GlobalContext) -> Result<(), anyhow::Error> {
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

fn handle_import(ctx: &mut GlobalContext, args: &ImportArgs) -> Result<(), anyhow::Error> {
    let content = fs::read_to_string(args.file.as_path())?;

    let data: HashSet<String> = content.lines().map(|s| s.to_string()).collect();
    let items: Vec<StudyItem> = data.iter().map(|d| StudyItem::from(d.to_owned())).collect();

    let list = StudyList::new(items);

    ctx.db_mut()?.insert_study_list(list)?;

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

fn handle_list(ctx: &mut GlobalContext, args: &ListArgs) -> Result<(), anyhow::Error> {
    let count = args.count.unwrap_or(0);

    let list = ctx
        .db_mut()?
        .get_study_list()
        .with_context(|| "failed to get study list")?;

    let mut items: Vec<StudyItem> = if count > 0 {
        list.items.iter().take(count).cloned().collect()
    } else {
        list.items
    };

    // TODO: use the enum directly (currently an invalid value will be converted to
    // "new")
    if let Some(status) = &args.status {
        items = items
            .iter()
            .filter(|i| i.status == status.as_str().into())
            .cloned()
            .collect();
    }

    for item in items {
        println!("{:?}", item);
    }

    Ok(())
}
