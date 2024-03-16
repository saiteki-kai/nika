use anyhow::Error;
use anyhow::Result;

use crate::core::study_list_manager::StudyListManager;

pub trait CommandHandler {
    fn handle(&self) -> Result<(), Error>;
}

pub trait StudyCommandHandler {
    fn handle(&self, manager: &mut StudyListManager) -> Result<(), Error>;
}
