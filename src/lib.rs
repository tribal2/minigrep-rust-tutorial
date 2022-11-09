mod search;

use std::error::Error;
use std::{fs, env};

pub fn app(cfg: Config) -> Result<(), Box<dyn Error>> {
    println!("\nSearching for << {:?} >>", &cfg.query);
    println!("on file: << {:?} >>", &cfg.filepath);

  // Obtenemos texto del archivo
  let file_txt = fs::read_to_string(&cfg.filepath)?;

  let results = search::search_in_string(&cfg.query, &file_txt);

  println!("\nResults:\n{:?}", results);

  Ok(())
}


pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn from_args() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() != 3 {
            return Err("You must provide 2 arguments");
        }

        let cfg = Config{
            query: args[1].clone(),
            filepath: args[2].clone()
        };

        Ok(cfg)
    }
}