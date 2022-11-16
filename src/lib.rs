pub mod search;

use std::error::Error;
use std::fs;

pub fn app(cfg: Config) -> Result<(), Box<dyn Error>> {
    if cfg.verbose {
        println!("\nSearching for << {:?} >>", &cfg.query);
        println!("on file: << {:?} >>", &cfg.file_path);
    }

    // Obtenemos texto del archivo
    let file_txt = fs::read_to_string(&cfg.file_path)?;

    let results = search::search_in_string(&cfg.query, &file_txt, cfg.case_sensitive);

    if cfg.verbose { println!("\nResults:\n"); }

    println!("{:?}", results);

    Ok(())
}


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
    pub verbose: bool,
}
