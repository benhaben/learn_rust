use rand::Rng;
use std::cmp::Ordering;
use std::io; //trait
fn main() {
    println!("猜数!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("神秘数字是 {}", secret_number);

    println!("猜测一个数!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    //shadow
    let guess: u32 = guess.trim().parse::<u32>().expect("please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"), //arm
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("Equal"),
    }
    println!("你猜的数是：{}", guess);
}
