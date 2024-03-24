use anyhow::Error;
use anyhow::Result;
use clap::Args;
use clap::Subcommand;
use nika_core::controllers::study_controller::StudyController;
use nika_core::repositories::config_repository::ConfigRepository;
use nika_core::repositories::dictionary_repository::DictionaryRepository;
use nika_core::repositories::list_repository::ListRepository;

use super::study_commands::AddArgs;
use super::study_commands::DailyArgs;
use super::study_commands::ListArgs;
use super::study_commands::MarkArgs;
use super::study_commands::RemoveArgs;
use super::study_commands::SelectArgs;
use super::study_commands::SetArgs;
use super::study_commands::ShowArgs;
use crate::config::app_config_dir;
use crate::config::app_data_dir;
use crate::config::CONFIG_NAME;
use crate::config::TAGS_BIN_PATH;
use crate::config::WORDS_BIN_PATH;
use crate::handlers::CommandHandler;
use crate::handlers::StudyCommandHandler;

#[derive(Subcommand)]
pub enum StudyCommands {
    List(ListArgs),
    Select(SelectArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    Daily(DailyArgs),
    Mark(MarkArgs),
    Show(ShowArgs),
    Set(SetArgs),
}

#[derive(Args)]
pub struct StudyArgs {
    #[command(subcommand)]
    commands: StudyCommands,
}

impl CommandHandler for StudyArgs {
    fn handle(&self) -> Result<(), Error> {
        let lists_path = app_data_dir().join("lists");
        let config_path = app_config_dir().join(format!("{}.toml", CONFIG_NAME));

        let dictionary_repository =
            DictionaryRepository::new(&WORDS_BIN_PATH.as_path(), &TAGS_BIN_PATH.as_path())?;
        let config_repository = ConfigRepository::new(config_path);
        let list_repository = ListRepository::new(lists_path);

        let study_controller =
            StudyController::new(dictionary_repository, config_repository, list_repository);

        match &self.commands {
            StudyCommands::Mark(args) => args.handle(&study_controller),
            StudyCommands::Daily(args) => args.handle(&study_controller),
            StudyCommands::Add(args) => args.handle(&study_controller),
            StudyCommands::Select(args) => args.handle(&study_controller),
            StudyCommands::Remove(args) => args.handle(&study_controller),
            StudyCommands::List(args) => args.handle(&study_controller),
            StudyCommands::Show(args) => args.handle(&study_controller),
            StudyCommands::Set(args) => args.handle(&study_controller),
        }
    }
}
