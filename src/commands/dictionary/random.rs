use clap::Args;

use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::utils::display::{print_word, DisplayMode};

#[derive(Args)]
pub struct RandomArgs {
    /// The number of random words to show
    #[arg(default_value_t = 1)]
    count: usize,

    /// Exclude uncommon words
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    exclude_uncommon: bool,

    /// Tags to filter by
    #[arg(short, long)]
    tags: Vec<String>,

    /// Show more details about the word
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

pub fn handle_random(ctx: &GlobalContext, args: &RandomArgs) -> CliResult<()> {
    let words = ctx.dictionary()?.random_words(args.count);

    for word in words {
        print_word(word, DisplayMode::Short);
    }

    Ok(())
}
