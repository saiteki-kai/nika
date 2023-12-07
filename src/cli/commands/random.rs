use clap::{Args, ValueEnum};

use crate::cli::app::dictionary;
use crate::cli::commands::CommandHandler;
use crate::cli::utils::display::{print_word, DisplayMode};

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
    number: Option<usize>,
}

impl CommandHandler for RandomArgs {
    fn handle(&self) {
        match self.option {
            RandomOption::Word => {
                let words = dictionary().random_words(self.number.unwrap_or(1));

                for word in words {
                    print_word(word, DisplayMode::Short);
                }
            }
            RandomOption::Kanji => println!("random kanji"),
        }
    }
}
