use clap::Args;

#[derive(Args)]
pub struct SearchArgs {
    /// The word to lookup
    query: Option<String>,
}

pub fn search(args: &SearchArgs) {
    match args.query {
        Some(ref _query) => {
            println!("Looking for {} ...", _query);
        }
        None => {
            println!("Please provide a word to lookup");
        }
    }
}
