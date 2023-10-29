use clap::{Args, ValueEnum};

use crate::cli::commands::CommandHandler;

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
}

impl CommandHandler for RandomArgs {
    fn handle(&self) {
        match self.option {
            RandomOption::Word => println!("random word"),
            RandomOption::Kanji => println!("random kanji"),
        }
    }
}
