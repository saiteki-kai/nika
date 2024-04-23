use anyhow::Context;
use clap::Args;
use nika_core::models::study::DailyItem;

use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::messages::*;
use crate::utils::status::WordStatus;

#[derive(Args)]
pub struct ListArgs {
    /// Limit the number of words to show
    #[arg(short, long)]
    count: Option<usize>,

    /// Show only the words in a specific status
    #[arg(short, long)]
    status: Option<WordStatus>,
}

enum OutputResult {
    Empty,
    Results(Vec<DailyItem>),
    NoResults,
}

fn filter_list(
    mut items: Vec<DailyItem>,
    count: Option<usize>,
    status: Option<WordStatus>,
) -> OutputResult {
    if items.is_empty() {
        return OutputResult::Empty;
    }

    if let Some(status) = &status {
        items = items
            .iter()
            .filter(|i| i.progress.status == status.into())
            .cloned()
            .collect();
    }

    if let Some(count) = count {
        items = items.iter().take(count).cloned().collect();
    }

    if !items.is_empty() {
        return OutputResult::Results(items);
    }

    OutputResult::NoResults
}

pub fn handle_list(ctx: &GlobalContext, args: &ListArgs) -> CliResult<()> {
    let list = ctx
        .db()?
        .get_daily_list()
        .with_context(|| "failed to get study list")?;

    match filter_list(list.items, args.count, args.status.clone()) {
        OutputResult::Empty => {
            println!("{}", DAILY_LIST_EMPTY);
        }
        OutputResult::NoResults => {
            println!("{}", DAILY_LIST_NO_RESULTS);
        }
        OutputResult::Results(list) => {
            for item in list {
                println!("{:?}", item);
            }
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use nika_core::models::study::{DailyItem, Status};
    use test_case::test_case;

    use super::*;

    #[test_case(None, 3)]
    #[test_case(Some(1), 1)]
    #[test_case(Some(2), 2)]
    #[test_case(Some(3), 3)]
    fn test_list_results_count(count: Option<usize>, result: usize) {
        let list = vec![
            DailyItem::from("a".to_owned(), 0, Status::New),
            DailyItem::from("b".to_owned(), 1, Status::Skipped),
            DailyItem::from("c".to_owned(), 2, Status::Done),
        ];

        let output = filter_list(list, count, None);

        match output {
            OutputResult::Results(items) => assert_eq!(items.len(), result),
            _ => panic!("Unexpected result"),
        }
    }

    #[test_case(WordStatus::New, 3)]
    #[test_case(WordStatus::Skipped, 2)]
    #[test_case(WordStatus::Discarded, 1)]
    #[test_case(WordStatus::Done, 2)]
    fn test_list_results_status(status: WordStatus, result: usize) {
        let list = vec![
            DailyItem::from("a".to_owned(), 0, Status::New),
            DailyItem::from("b".to_owned(), 1, Status::Skipped),
            DailyItem::from("c".to_owned(), 2, Status::Done),
            DailyItem::from("d".to_owned(), 3, Status::Done),
            DailyItem::from("e".to_owned(), 4, Status::New),
            DailyItem::from("f".to_owned(), 5, Status::Skipped),
            DailyItem::from("g".to_owned(), 6, Status::Discarded),
            DailyItem::from("h".to_owned(), 7, Status::New),
        ];

        let output = filter_list(list, None, Some(status));

        match output {
            OutputResult::Results(items) => assert_eq!(items.len(), result),
            _ => panic!("Unexpected result"),
        }
    }

    #[test_case(None, None)]
    #[test_case(Some(0), None)]
    #[test_case(Some(1), None)]
    #[test_case(None, Some(WordStatus::New))]
    #[test_case(None, Some(WordStatus::Skipped))]
    #[test_case(None, Some(WordStatus::Discarded))]
    #[test_case(Some(2), Some(WordStatus::Done))]
    fn test_list_no_items(count: Option<usize>, status: Option<WordStatus>) {
        let output = filter_list(vec![], count, status);
        assert!(matches!(output, OutputResult::Empty));
    }
}
