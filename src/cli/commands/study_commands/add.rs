use clap::Args;

use crate::cli::commands::CommandHandler;

#[derive(Args)]
pub struct AddArgs {
    name: String,
    file: String,
}

impl CommandHandler for AddArgs {
    fn handle(&self) {}
}
