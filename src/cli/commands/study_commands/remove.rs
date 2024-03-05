use clap::Args;

use crate::cli::commands::CommandHandler;

#[derive(Args)]
pub struct RemoveArgs {
    name: String,
}

impl CommandHandler for RemoveArgs {
    fn handle(&self) {}
}
