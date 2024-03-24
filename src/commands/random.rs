use anyhow::Error;
use anyhow::Result;
use clap::Args;
use clap::ValueEnum;
use nika_core::controllers::random_controller::RandomController;
use nika_core::repositories::dictionary_repository::DictionaryRepository;

use crate::config::TAGS_BIN_PATH;
use crate::config::WORDS_BIN_PATH;
use crate::handlers::CommandHandler;
use crate::utils::display::print_word;
use crate::utils::display::DisplayMode;

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
        let dictionary_repository =
            DictionaryRepository::new(&WORDS_BIN_PATH.as_path(), &TAGS_BIN_PATH.as_path())?;
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
