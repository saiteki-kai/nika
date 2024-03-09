use clap::Args;

use crate::cli::commands::CommandHandler;
use crate::cli::utils::links::{generate_hyperlink, get_links};
use crate::core::words::{daily_words, Word};
use crate::utils::Link;

#[derive(Args)]
pub struct DailyArgs {
    /// Show only the words
    #[arg(short = 's', long = "summary")]
    summary: bool,
}

impl CommandHandler for DailyArgs {
    fn handle(&self) {
        if self.summary {
            println!("NIKA • Today's Summary:\n");

            let words = daily_words();
            let links = get_links();

            words.iter().enumerate().for_each(|(index, x)| {
                let entry = summary_word_entry(x, &links);
                println!("{}. {}", index + 1, entry);
            });
            println!();

            return;
        }

        println!("Japanese Daily Words\n");

        // definitions from dictionary
        // examples from tatoeba
        // links to online dictionaries

        println!("1. 放す (はなす)");
        println!("   to release; to let go; to free; to set free; to let loose; to turn loose");
        println!("   ");
    }
}

fn summary_word_entry(word: &Word, links: &[Link]) -> String {
    let word_fmt = format!("{} ({})", word.text, word.reading);

    let hyperlinks = links
        .iter()
        .map(|link| {
            let url = format!("{}{}", link.base_url, word.text);
            generate_hyperlink(&link.text, &url)
        })
        .collect::<Vec<String>>()
        .join(" ");

    format!("{:<24}\t{:<20}", word_fmt, hyperlinks)
}
