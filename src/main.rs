use rand::Rng;
use std::cmp::Ordering;
use std::io; //trait
fn main() {
    println!("猜数!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("神秘数字是 {}", secret_number);

    loop {
        println!("-------------\n猜测一个数!");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        //shadow
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"), //arm
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
        println!("你猜的数是：{}", guess);
    }
}
