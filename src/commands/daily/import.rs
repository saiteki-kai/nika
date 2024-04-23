use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use clap::Args;
use nika_core::models::study::{DailyItem, DailyList};

use crate::context::GlobalContext;
use crate::error::CliResult;

#[derive(Args)]
pub struct ImportArgs {
    /// The file to import
    #[arg(required = true)]
    file: PathBuf,
}

pub fn handle_import(ctx: &GlobalContext, args: &ImportArgs) -> CliResult<()> {
    // call import function to dispatch to the correct handler
    let content = fs::read_to_string(args.file.as_path())?;

    // with queries
    // let data = serde_json::from_str::<Vec<Query>>(&content)?;

    // perform matching
    // let items =

    // with ids
    // ! (do not use the hashset, you lose the order)
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
