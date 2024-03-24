use anyhow::Error;
use anyhow::Result;
use nika_core::controllers::study_controller::StudyController;

pub trait CommandHandler {
    fn handle(&self) -> Result<(), Error>;
}

pub trait StudyCommandHandler {
    fn handle(&self, controller: &StudyController) -> Result<(), Error>;
}
