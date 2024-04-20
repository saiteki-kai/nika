use anyhow::Context;
use anyhow::Ok;
use clap::Args;
use clap::Subcommand;
use nika_core::models::dictionary::jmdict::Kana;
use nika_core::models::dictionary::jmdict::Kanji;
use nika_core::models::dictionary::jmdict::Sense;
use nika_core::models::dictionary::jmdict::Word;
use nika_core::preferences::Link;

use crate::context::GlobalContext;
use crate::error::CliResult;
use crate::handlers::CommandHandler;
use crate::utils::links::generate_hyperlink;

#[derive(Subcommand)]
enum StudyCommand {
    /// Study new words from daily and discovery lists
    New(NewArgs),
    /// Review the words you have learned
    Review,
    /// Mark a word as learned manually
    Mark,
    /// Show your progress
    Progress,
}

#[derive(Args)]
pub struct StudyArgs {
    #[command(subcommand)]
    commands: StudyCommand,
}

impl CommandHandler for StudyArgs {
    fn handle(&self, ctx: &GlobalContext) -> CliResult<()> {
        match &self.commands {
            StudyCommand::New(args) => handle_new(ctx, args),
            StudyCommand::Review => Ok(println!("not implemented yet")),
            StudyCommand::Mark => Ok(println!("not implemented yet")),
            StudyCommand::Progress => Ok(println!("not implemented yet")),
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum StudyListOption {
    Daily,
    Discovery,
    Both,
}

#[derive(Args)]
pub struct NewArgs {
    /// The name of the list to study
    #[arg(short, long, value_enum)]
    name: Option<StudyListOption>,

    /// Show only the words
    #[arg(short = 's', long = "summary")]
    summary: bool,
}

fn handle_new(ctx: &GlobalContext, args: &NewArgs) -> CliResult<()> {
    let links = &ctx.prefs()?.external_dictionaries;
    let words = study_words(ctx, true)?;

    if args.summary {
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

pub fn study_words(ctx: &GlobalContext, daily: bool) -> CliResult<Vec<&Word>> {
    let items = ctx
        .db()?
        .get_study_list()
        .with_context(|| "failed to get study list")?;

    let index = 0; // study_list.config.current_index;
    let count = 5; // study_list.config.items_per_day;

    let id_list = items.iter().map(|item| item.word_id.as_str());

    let ids: Vec<&str> = if !daily {
        id_list.collect()
    } else {
        id_list.skip(index).take(count).collect()
    };

    let words = ctx.dictionary()?.words(&ids);

    Ok(words)
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
