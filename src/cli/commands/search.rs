use super::CommandHandler;
use clap::Args;

#[derive(Args)]
pub struct SearchArgs {
    /// The word to lookup
    query: Option<String>,
}

impl CommandHandler for SearchArgs {
    fn handle(&self) {
        match self.query {
            Some(ref _query) => {
                println!("Looking for {} ...", _query);
            }
            None => {
                println!("Please provide a word to lookup");
            }
        }
    }
}
