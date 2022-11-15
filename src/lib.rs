pub mod search;

use std::error::Error;
use std::{fs, env};

pub fn app(cfg: Config) -> Result<(), Box<dyn Error>> {
    println!("\nSearching for << {:?} >>", &cfg.query);
    println!("on file: << {:?} >>", &cfg.file_path);

  // Obtenemos texto del archivo
  let file_txt = fs::read_to_string(&cfg.file_path)?;

  let results = search::search_in_string(&cfg.query, &file_txt);

  println!("\nResults:\n{:?}", results);

  Ok(())
}


pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn from_args() -> Result<Config, &'static str> {
        let mut args = env::args();

        // if args.len() != 3 {
        //     return Err("You must provide 2 arguments");
        // }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config{
            query,
            file_path,
        })
    }
}
