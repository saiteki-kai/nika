use anyhow::Error;
use anyhow::Result;
use clap::Args;
use clap::ValueEnum;

use crate::cli::handlers::CommandHandler;
use crate::cli::utils::display::print_word;
use crate::cli::utils::display::DisplayMode;
use crate::core::controllers::random_controller::RandomController;
use crate::core::repositories::dictionary_repository::DictionaryRepository;

#[derive(Clone, ValueEnum)]
pub enum RandomOption {
    Word,
    Kanji,
}

#[derive(Args)]
pub struct RandomArgs {
    /// Specify what to generate
    #[arg(value_enum)]
    option: RandomOption,
    /// Number of words/kanji to generate
    count: Option<usize>,
}

impl CommandHandler for RandomArgs {
    fn handle(&self) -> Result<(), Error> {
        let dictionary_repository = DictionaryRepository::new()?;
        let controller = RandomController::new(dictionary_repository);

        match self.option {
            RandomOption::Word => {
                let words = controller.random_words(self.count.unwrap_or(1));

                for word in words {
                    print_word(word, DisplayMode::Short);
                }
            }
            RandomOption::Kanji => println!("random kanji"),
        }

        Ok(())
    }
}
