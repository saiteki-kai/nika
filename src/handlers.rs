use anyhow::Error;
use anyhow::Result;

use crate::context::GlobalContext;

pub trait CommandHandler {
    fn handle(&self, ctx: &mut GlobalContext) -> Result<(), Error>;
}
