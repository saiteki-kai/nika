use clap::Args;

#[derive(Args)]
pub struct DailyArgs {
    /// Show more details
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

pub fn daily(args: &DailyArgs) {
    println!("Words for today");

    if args.verbose {
        println!("Verbose");
    }
}
