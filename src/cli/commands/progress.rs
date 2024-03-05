use clap::Args;

use super::CommandHandler;

#[derive(Args)]
pub struct ProgressArgs {
    /// Show more information
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

impl CommandHandler for ProgressArgs {
    fn handle(&self) {
        if self.verbose {
            println!("Verbose");
        }
        println!("your progress summary");
    }
}
