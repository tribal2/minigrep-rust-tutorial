use std::process;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query
    query: String,

    /// File to search
    file_path: String,

    /// Flag to activate case sensitive search
    #[arg(short, long, default_value_t = false)]
    case_sensitive: bool,
}

fn main() {
    // Obtenemos argumentos
    let config = minigrep::Config::from_args()
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(1);
        });

    if let Err(e) = minigrep::app(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
