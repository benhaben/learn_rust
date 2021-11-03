use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect(); //不支持非法Unicode
    println!("{:?}", args);

    let config = Config::new(&args);

    println!("search for {}", config.query);
    println!("in file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something wrong!!!");

    println!("with test:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

fn parse(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

// 二进制程序关注点分离的指导原则
// 将程序拆分为main.rs和lib.rs，将业务逻辑放到lib.rs
// 当命令行解析逻辑较少时，将它放在main.rs也行
// 当命令行解析逻辑变复杂时，需要将它从main.rs提取到lib.rs
