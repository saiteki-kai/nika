use crate::context::GlobalContext;
use crate::error::CliResult;

pub trait CommandHandler {
    fn handle(&self, ctx: &GlobalContext) -> CliResult<()>;
}
