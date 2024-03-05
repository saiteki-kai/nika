use clap::Args;

use crate::cli::commands::CommandHandler;

#[derive(Args)]
pub struct ListArgs {
    name: String,
}

impl CommandHandler for ListArgs {
    fn handle(&self) {}
}
