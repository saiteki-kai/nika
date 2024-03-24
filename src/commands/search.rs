use anyhow::Error;
use anyhow::Result;
use clap::Args;
use nika_core::controllers::search_controller::SearchController;
use nika_core::repositories::dictionary_repository::DictionaryRepository;

use crate::config::TAGS_BIN_PATH;
use crate::config::WORDS_BIN_PATH;
use crate::handlers::CommandHandler;

#[derive(Args)]
pub struct SearchArgs {
    /// The word to lookup
    query: Option<String>,

    #[arg(short = 'c', long = "common")]
    common: Option<bool>,
}

impl CommandHandler for SearchArgs {
    fn handle(&self) -> Result<(), Error> {
        let dictionary_repository =
            DictionaryRepository::new(&WORDS_BIN_PATH.as_path(), &TAGS_BIN_PATH.as_path())?;
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
