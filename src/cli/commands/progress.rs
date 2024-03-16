use anyhow::Error;
use anyhow::Result;
use clap::Args;

use crate::cli::handlers::CommandHandler;

#[derive(Args)]
pub struct ProgressArgs {
    /// Show more information
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

impl CommandHandler for ProgressArgs {
    fn handle(&self) -> Result<(), Error> {
        if self.verbose {
            println!("Verbose");
        }
        println!("your progress summary");

        Ok(())
    }
}
