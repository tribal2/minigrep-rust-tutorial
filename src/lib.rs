pub mod search;

use std::error::Error;
use std::fs;
use clap;
use serde_json::json;

pub fn app(cfg: Config) -> Result<(), Box<dyn Error>> {
    if cfg.verbose {
        println!("\nSearching for << {:?} >>", &cfg.query);
        println!("on file: << {:?} >>", &cfg.file_path);
    }

    // Obtenemos texto del archivo
    let file_txt = fs::read_to_string(&cfg.file_path)?;

    let results = search::search_in_string(&cfg.query, &file_txt, cfg.case_sensitive);

    if cfg.verbose { println!("\nResults:\n"); }

    show_results(results, cfg.output);

    Ok(())
}

fn show_results(results: Vec<&str>, mode: OutputMode) {
    match mode {
        OutputMode::Lines => {
            for line in results {
                println!("{}", line);
            }
        },
        OutputMode::Json => {
            let json = json!(results);
            println!("{}", json.to_string());
        },
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputMode {
    Lines,
    Json,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub output: OutputMode,
    pub case_sensitive: bool,
    pub verbose: bool,
}
