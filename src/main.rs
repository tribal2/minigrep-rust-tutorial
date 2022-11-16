use std::process;
use clap::Parser;
use minigrep::{Config, OutputMode};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query
    query: String,

    /// File to search
    file_path: String,

    /// Flag to activate output as JSON encoded string
    #[arg(
        value_enum,
        short,
        long,
        default_value_t = OutputMode::Lines,
    )]
    output: OutputMode,

    /// Flag to activate case sensitive search
    #[arg(short, long, default_value_t = false)]
    case_sensitive: bool,

    /// Flag to activate verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let config = Config {
        query: args.query,
        file_path: args.file_path,
        output: args.output,
        case_sensitive: args.case_sensitive,
        verbose: args.verbose,
    };

    if let Err(e) = minigrep::app(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
