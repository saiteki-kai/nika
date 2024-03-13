use anyhow::{Error, Ok, Result};
use clap::Args;

use crate::cli::app::dictionary;
use crate::cli::commands::study_commands::utils::get_list_name;
use crate::cli::handlers::StudyCommandHandler;
use crate::cli::utils::links::{generate_hyperlink, get_links};
use crate::core::models::jmdict::Word;
use crate::core::models::link::Link;
use crate::core::study_list_manager::StudyListManager;

#[derive(Args)]
pub struct DailyArgs {
    #[arg(short = 'n', long = "name")]
    name: Option<String>,
    /// Show only the words
    #[arg(short = 's', long = "summary")]
    summary: bool,
}

impl StudyCommandHandler for DailyArgs {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error> {
        let list_name: String = get_list_name(self.name.clone(), manager.current.clone())?;

        if self.summary {
            println!("NIKA • Today's Summary:\n");

            let ids = manager.study(&list_name, true)?;
            let words = dictionary().words(&ids.iter().map(AsRef::as_ref).collect::<Vec<&str>>());

            let links = get_links();

            words.iter().enumerate().for_each(|(index, x)| {
                let entry = summary_word_entry(x, &links);
                println!("{}. {}", index + 1, entry);
            });
            println!();

            return Ok(());
        }

        println!("Japanese Daily Words\n");

        // definitions from dictionary
        // examples from tatoeba
        // links to online dictionaries

        println!("1. 放す (はなす)");
        println!("   to release; to let go; to free; to set free; to let loose; to turn loose");
        println!("   ");

        Ok(())
    }
}

fn summary_word_entry(word: &Word, links: &[Link]) -> String {
    let text: String;
    let reading: Option<String>;

    if !word.kanji.is_empty() {
        text = word.kanji[0].clone().text;
        reading = Some(word.kana[0].clone().text);
    } else {
        text = word.kana[0].clone().text;
        reading = None;
    }

    let word_fmt = if reading.is_none() {
        text.clone()
    } else {
        format!("{} ({})", text, reading.unwrap())
    };

    let hyperlinks = links
        .iter()
        .map(|link| {
            let url = format!("{}{}", link.base_url, text);
            generate_hyperlink(&link.text, &url)
        })
        .collect::<Vec<String>>()
        .join(" ");

    format!("{:<24}\t{:<20}", word_fmt, hyperlinks)
}
