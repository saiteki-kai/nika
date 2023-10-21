use super::lib::CommandHandler;
use clap::Args;

#[derive(Args)]
pub struct DailyArgs {
    /// Show more details
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

impl CommandHandler for DailyArgs {
    fn handle(&self) {
        println!("Words for today");

        if self.verbose {
            println!("Verbose");
        }
    }
}
