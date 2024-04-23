use std::path::PathBuf;

use clap::Args;

use crate::context::GlobalContext;
use crate::error::CliResult;

#[derive(Args)]
pub struct ImportArgs {
    /// The file to import
    #[arg(required = true)]
    file: PathBuf,
}

pub fn handle_import(_ctx: &GlobalContext, _args: &ImportArgs) -> CliResult<()> {
    println!("not implemented yet");
    Ok(())
}
