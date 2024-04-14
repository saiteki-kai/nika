use anyhow::Error;
use anyhow::Result;

use crate::context::Context;

pub trait CommandHandler {
    fn handle(&self, ctx: &Context) -> Result<(), Error>;
}
