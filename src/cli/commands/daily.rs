use crate::cli::utils::links::{generate_hyperlink, get_links, Link};
use crate::core;

use super::lib::CommandHandler;

use clap::{Args, Subcommand};
use core::repository::words::{daily_words, Word};

#[derive(Subcommand)]
enum DailyCommands {
    Mark(MarkArgs),
}

#[derive(Args)]
struct MarkArgs {
    word: String,
    status: String, // TODO: enum
}

#[derive(Args)]
pub struct DailyArgs {
    #[command(subcommand)]
    commands: Option<DailyCommands>,

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
        match &self.commands {
            Some(DailyCommands::Mark(args)) => {
                println!("{:?} marked as {:?}", args.word, args.status)
            }
            _ => {
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
                println!(
                    "   to release; to let go; to free; to set free; to let loose; to turn loose"
                );
                println!("   ");
            }
        }
    }
}
