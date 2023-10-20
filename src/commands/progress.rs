use clap::Args;

#[derive(Args)]
pub struct ProgressArgs {
    /// Show more information
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

pub fn progress(args: &ProgressArgs) {
    if args.verbose {
        println!("Verbose");
    }
    println!("your progress summary");
}
