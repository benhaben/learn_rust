use std::{env, error::Error, fs,  process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("search for {}", config.query);
    println!("in file {}", config.filename);
    let contents = fs::read_to_string(config.filename)?;
    println!("with test:\n{}", contents);
    Ok(())

}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&'static str> {

        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
         Ok(Config { query, filename })
    }
}

pub fn parse(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}