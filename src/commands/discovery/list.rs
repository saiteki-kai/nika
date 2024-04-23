use clap::Args;

use crate::context::GlobalContext;
use crate::error::CliResult;

#[derive(Args)]
pub struct ListArgs {
    /// Show all the words in the discovery list
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    all: Option<bool>,

    /// Limit the number of words to show
    #[arg(short, long)]
    count: Option<usize>,

    /// Show only the words in a specific status
    #[arg(short, long)]
    status: Option<String>,
}

pub fn handle_list(_ctx: &GlobalContext, _args: &ListArgs) -> CliResult<()> {
    println!("not implemented yet");
    Ok(())
}
