use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();//不支持非法Unicode
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("search for {}",query);
    println!("in file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something wrong!!!");

    println!("with test:\n{}", contents);

}
