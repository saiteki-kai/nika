use std::fs;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::StudyCommandHandler;
use crate::core::controllers::study_controller::StudyController;
use crate::core::errors::ErrorKind;
use crate::core::errors::StudyListError;
use crate::core::models::study_list::StudyList;

#[derive(Args)]
pub struct AddArgs {
    name: String,
    file: PathBuf,
}

impl StudyCommandHandler for AddArgs {
    fn handle(&self, controller: &StudyController) -> Result<(), Error> {
        // TODO: ask for overwrite when the name is already present!
        // TODO: allow to pass the default value for the number of words per day

        let items = fs::read_to_string(&self.file)?
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();

        let study_list = StudyList::new(&self.name, items);

        let is_empty = controller.lists()?.is_empty();

        match controller.add(study_list) {
            Ok(_) => Ok::<(), ErrorKind>(()),
            Err(error) => {
                return match error {
                    ErrorKind::List(StudyListError::ListAlreadyExists) => {
                        eprintln!("List '{}' already exists", &self.name);

                        // TODO: ask for overwrite

                        Ok(())
                    }
                    _ => Err(error.into()),
                };
            }
        }?;

        if is_empty {
            controller.select(&self.name)?;
        }

        Ok(())
    }
}
