use clap::Args;

use crate::cli::app::dictionary;

use super::CommandHandler;

#[derive(Args)]
pub struct SearchArgs {
    /// The word to lookup
    query: Option<String>,

    #[arg(short = 'c', long = "common")]
    common: Option<bool>,
}

impl CommandHandler for SearchArgs {
    fn handle(&self) {
        match self.query {
            Some(ref _query) => {
                let results = dictionary().search(_query, self.common);

                println!("{} Results found for {}\n", results.len(), _query);

                for res in &results {
                    println!("{:?}\n", res);
                }
            }
            None => {
                println!("Please provide a word to lookup");
            }
        }
    }
}
