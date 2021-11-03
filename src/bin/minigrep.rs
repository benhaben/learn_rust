use std::{env, error::Error, fs,  process};
use guessing_game::minigrep::{Config};

fn main() {
    let args: Vec<String> = env::args().collect(); //不支持非法Unicode
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("error args:{}", err);
        process::exit(1);//这样就不会打印很多没用的错误
    });
    
    if let Err(err) = guessing_game::minigrep::run(config){
        println!("error run:{}", err);
        process::exit(1);//这样就不会打印很多没用的错误
    }
}


// 二进制程序关注点分离的指导原则
// 将程序拆分为main.rs和lib.rs，将业务逻辑放到lib.rs
// 当命令行解析逻辑较少时，将它放在main.rs也行
// 当命令行解析逻辑变复杂时，需要将它从main.rs提取到lib.rs
