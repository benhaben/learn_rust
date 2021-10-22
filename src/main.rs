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

    //shadowing
    let x=5;
    let x= x+1;
    let x:u32 = x; //类型可以变

    //标量类型：
    //整数,u32,i32,i8,i16,i32,i64,i128,isize，无符号就是u
    //整数字面值：98_000, 0xff, 0o33,0b1111_0000,b'A'
    //整数溢出：编译时会报错，release不会检查，进行环绕，u8,256变成0

    //浮点类型：
    //f32,f64(默认类型)
    let x = 2.0;//f64
    let x:f32 = 2.0;

    //rust有类型推断

    //bool: true,fasle,一个字节
    let x= true;

    // 字符类型 char，四个字节 unicode,表情也可以
    let x= 'x';
    let x = '牙';
    let x = '😄';
    
    // tuple


    println!("x：{}", x);

}

const MAX_POINT: u32 = 1000;
