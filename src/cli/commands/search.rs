use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::CommandHandler;
use crate::core::controllers::search_controller::SearchController;
use crate::core::repositories::dictionary_repository::DictionaryRepository;

#[derive(Args)]
pub struct SearchArgs {
    /// The word to lookup
    query: Option<String>,

    #[arg(short = 'c', long = "common")]
    common: Option<bool>,
}

impl CommandHandler for SearchArgs {
    fn handle(&self) -> Result<(), Error> {
        let dictionary_repository = DictionaryRepository::new()?;
        let controller = SearchController::new(dictionary_repository);

        match self.query {
            Some(ref query) => {
                let results = controller.search(query, self.common);

                println!("{} Results found for {}\n", results.len(), query);

                for res in &results {
                    println!("{:?}\n", res);
                }
            }
            None => {
                println!("Please provide a word to lookup");
            }
        }

        Ok(())
    }
}
