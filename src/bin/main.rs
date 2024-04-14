use nika::cli::run;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    std::process::exit(0);
}
