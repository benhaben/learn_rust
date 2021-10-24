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
    let x = 5;
    let x = x + 1;
    let x: u32 = x; //类型可以变

    //标量类型：
    //整数,u32,i32,i8,i16,i32,i64,i128,isize，无符号就是u
    //整数字面值：98_000, 0xff, 0o33,0b1111_0000,b'A'
    //整数溢出：编译时会报错，release不会检查，进行环绕，u8,256变成0

    //浮点类型：
    //f32,f64(默认类型)
    let x = 2.0; //f64
    let x: f32 = 2.0;

    //rust有类型推断

    //bool: true,fasle,一个字节
    let x = true;

    // 字符类型 char，四个字节 unicode,表情也可以
    let x = 'x';
    let x = '牙';
    let x = '😄';

    // 复合类型
    // tuple,长度申明指定，不能变
    let tup: (i32, f32) = (22, 22.22);
    let x = tup.1;
    let (x, y) = tup; //解构
    println!("x：{}， y:{}", x, y);

    // 数组，栈上。vector由标准库提供，比数组灵活
    let x = [3; 5]; // [3,3,3,3,3]
                    //越界，编译时候不会报错(简单的情况除外)，运行时候会panic，和c不一样
                    // let x = x[1000];
    println!("x：{}", x[1]);

    //函数
    //函数申明使用fn，使用snake case
    //单词之间用下划线
    another_function(7);//arguments
    //paramters 形参，类型必须指定; arguments 实参

    //函数体中语句和表达式
    // 语句是执行指令，代码，没有返回值
    // 表达式是有计算结果，比如 5+6
    let x = { x[1] + 1 };

   
    // let x = {
    //     x+1;  // error：语句没有返回值
    // };

    //函数返回值
    let x = five();
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {//paramters
    println!("another_function")
}

const MAX_POINT: u32 = 1000;
