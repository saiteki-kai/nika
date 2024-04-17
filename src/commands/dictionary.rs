use clap::Args;
use clap::Subcommand;

use crate::context::GlobalContext;
use crate::handlers::CommandHandler;
use crate::utils::display::print_word;
use crate::utils::display::DisplayMode;

#[derive(Subcommand)]
enum DictionaryCommand {
    /// Search for a word in the dictionary
    Search(SearchArgs),
    /// Get a random word from the dictionary
    Random(RandomArgs),
}

#[derive(Args)]
pub struct DictionaryArgs {
    #[command(subcommand)]
    commands: DictionaryCommand,
}

impl CommandHandler for DictionaryArgs {
    fn handle(&self, ctx: &mut GlobalContext) -> Result<(), anyhow::Error> {
        match &self.commands {
            DictionaryCommand::Search(args) => handle_search(ctx, args),
            DictionaryCommand::Random(args) => handle_random(ctx, args),
        }
    }
}

#[derive(Args)]
struct SearchArgs {
    /// The word to lookup
    query: Option<String>,

    /// Show only common words
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    common: Option<bool>,

    /// Show only uncommon words
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    uncommon: Option<bool>,

    /// Show more details about the word
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

fn handle_search(_ctx: &mut GlobalContext, _args: &SearchArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}

#[derive(Args)]
struct RandomArgs {
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

fn handle_random(ctx: &GlobalContext, args: &RandomArgs) -> Result<(), anyhow::Error> {
    let words = ctx.dictionary()?.random_words(args.count);

    for word in words {
        print_word(word, DisplayMode::Short);
    }

    Ok(())
}
