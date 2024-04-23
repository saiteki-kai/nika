use clap::Args;

use crate::context::GlobalContext;
use crate::error::CliResult;

#[derive(Args)]
pub struct SearchArgs {
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

pub fn handle_search(_ctx: &GlobalContext, _args: &SearchArgs) -> CliResult<()> {
    println!("not implemented yet");
    Ok(())
}
