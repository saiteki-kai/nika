use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use clap::Args;

use crate::cli::commands::study_commands::utils::get_list_name;
use crate::cli::handlers::StudyCommandHandler;
use crate::cli::utils::links::generate_hyperlink;
use crate::core::controllers::study_controller::StudyController;
use crate::core::models::jmdict::Kana;
use crate::core::models::jmdict::Kanji;
use crate::core::models::jmdict::Sense;
use crate::core::models::jmdict::Word;
use crate::core::models::link::Link;

#[derive(Args)]
pub struct DailyArgs {
    #[arg(short = 'n', long = "name")]
    name: Option<String>,
    /// Show only the words
    #[arg(short = 's', long = "summary")]
    summary: bool,
}

impl StudyCommandHandler for DailyArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        let list_name =
            get_list_name(self.name.as_deref(), controller.selected_list()?.as_deref())?;

        let words = controller.study_words(&list_name, true)?;
        let links = controller.get_links()?;

        if self.summary {
            println!("NIKA • Today's Summary:\n");

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

        for (i, word) in words.iter().enumerate() {
            detailed_print(i + 1, word, &links);
        }

        Ok(())
    }
}

fn print_links(text: &str, links: &[Link]) -> String {
    let hyperlinks = links
        .iter()
        .map(|link| {
            let url = format!("{}{}", link.base_url, text);
            generate_hyperlink(&link.text, &url)
        })
        .collect::<Vec<String>>()
        .join(" ");

    hyperlinks
}

fn print_sense(senses: &[Sense]) {
    let text = senses
        .iter()
        .enumerate()
        .map(|(i, s)| {
            format!(
                "   {}\n{}. {}",
                s.part_of_speech
                    //.iter()
                    //.map(|tag| dictionary().tag(tag).unwrap_or(tag).clone())
                    //.collect::<Vec<String>>()
                    .join(", "),
                i + 1,
                s.gloss
                    .iter()
                    .map(|g| g.text.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    for line in text.lines() {
        println!("   {}", line);
    }

    println!();
}

fn detailed_print(n: usize, word: &Word, links: &[Link]) {
    // if let Some((first, others)) = word.kanji.split_first() {
    //     println!("{}", first.text);
    // }

    let mut kanji_kana_map = Vec::<(&Kanji, &Kana)>::new();

    for kana in &word.kana {
        for a in &kana.applies_to_kanji {
            if a == "*" {
                for kanji in &word.kanji {
                    kanji_kana_map.push((kanji, kana));
                }
            } else if let Some(kanji) = &word.kanji.iter().find(|k| k.text == *a) {
                kanji_kana_map.push((*kanji, kana));
            }
        }
    }

    if let Some((first, others)) = kanji_kana_map.split_first() {
        println!("{}. {} 「{}」\n", n, first.0.text, first.1.text);

        print_sense(&word.sense);

        if !others.is_empty() {
            println!(
                "   Other forms:\n   {}\n",
                others
                    .iter()
                    .map(|l| format!("{} 【{}】", l.0.text, l.1.text))
                    .collect::<Vec<String>>()
                    .join("、")
            );
        }

        println!("   {}\n\n", print_links(&first.0.text, links));
    } else {
        let text = &word.kana.first().unwrap().text;

        println!("{}. {}\n", n, text);
        print_sense(&word.sense);
        println!("   {}\n\n", print_links(text, links));
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
