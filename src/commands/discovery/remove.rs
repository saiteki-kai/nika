use clap::Args;

use crate::context::GlobalContext;
use crate::error::CliResult;

#[derive(Args)]
pub struct RemoveArgs {
    /// Kanji or kana of the word
    text: Option<String>,
    /// Reading of the word
    reading: Option<String>,
    /// Meaning of the word
    meaning: Option<String>,
}

pub fn handle_remove(_ctx: &GlobalContext, _args: &RemoveArgs) -> CliResult<()> {
    println!("not implemented yet");
    Ok(())
}
