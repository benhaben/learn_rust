use std::{env, error::Error, fs, process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("search for {}", config.query);
    println!("in file {}", config.filename);
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    println!("with test:\n{}", contents);
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = args[1].clone(); //TODO: 使用迭代器
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn parse(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //todo： 使用迭代器
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // 零开销抽象，不会引入额外开销，反而更快
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        rust:
        safe,faster,productive.
        ";
        assert_ne!(vec!["safe,faster,productive."], search(query, contents));
    }
}
