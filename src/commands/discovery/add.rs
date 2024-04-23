use clap::Args;

use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::utils::status::WordStatus;

#[derive(Args)]
pub struct AddArgs {
    /// Kanji or kana of the word
    text: Option<String>,
    /// Reading of the word
    reading: Option<String>,
    /// Meaning of the word
    meaning: Option<WordStatus>,
}

pub fn handle_add(_ctx: &GlobalContext, _args: &AddArgs) -> CliResult<()> {
    println!("not implemented yet");
    Ok(())
}
