use crate::api;
use crate::utils::links::{generate_hyperlink, get_links, Link};

use super::lib::CommandHandler;

use api::words::{daily_words, Word};
use clap::Args;

#[derive(Args)]
pub struct DailyArgs {
    /// Show only the words
    #[arg(short = 's', long = "summary")]
    summary: bool,
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

impl CommandHandler for DailyArgs {
    fn handle(&self) {
        if self.summary {
            // let (_, cols) = terminal_size();

            // let repeated_chars = "#".repeat(cols.into());
            // println!("{}", repeated_chars);

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

        println!("Japanese Daily Words");

        // definitions from dictionary
        // examples from tatoeba
        // links to online dictionaries
    }
}
