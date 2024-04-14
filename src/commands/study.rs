use clap::Args;
use clap::Subcommand;

use crate::context::Context;
use crate::handlers::CommandHandler;

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
    fn handle(&self, ctx: &Context) -> Result<(), anyhow::Error> {
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
}

fn handle_new(_ctx: &Context, _args: &NewArgs) -> Result<(), anyhow::Error> {
    println!("not implemented yet");
    Ok(())
}
