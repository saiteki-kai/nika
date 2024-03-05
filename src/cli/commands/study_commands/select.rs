use clap::Args;

use crate::cli::commands::CommandHandler;

#[derive(Args)]
pub struct SelectArgs {
    name: String,
}

impl CommandHandler for SelectArgs {
    fn handle(&self) {}
}
