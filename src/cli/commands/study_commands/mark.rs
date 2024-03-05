use clap::Args;

use crate::cli::commands::CommandHandler;

#[derive(Args)]
pub struct MarkArgs {
    pub word: String,
    pub status: String,
}

impl CommandHandler for MarkArgs {
    fn handle(&self) {
        println!("{:?} marked as {:?}", self.word, self.status)
    }
}
