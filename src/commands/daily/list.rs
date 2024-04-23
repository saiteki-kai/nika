use anyhow::Context;
use clap::Args;
use nika_core::models::study::DailyItem;

use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::messages::*;
use crate::utils::status::WordStatus;

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
    status: Option<WordStatus>,
}

pub fn handle_list(ctx: &GlobalContext, args: &ListArgs) -> CliResult<()> {
    let count = args.count.unwrap_or(0);

    let list = ctx
        .db()?
        .get_daily_list()
        .with_context(|| "failed to get study list")?;

    if list.is_empty() {
        println!("{}", DAILY_LIST_EMPTY);

        return Ok(());
    }

    let mut items: Vec<DailyItem> = if count > 0 {
        list.items.iter().take(count).cloned().collect()
    } else {
        list.items
    };

    if let Some(status) = &args.status {
        items = items
            .iter()
            .filter(|i| i.progress.status == status.into())
            .cloned()
            .collect();
    }

    if items.is_empty() {
        println!("{}", DAILY_LIST_NO_RESULTS);

        return Ok(());
    }

    for item in items {
        println!("{:?}", item);
    }

    Ok(())
}
