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
    another_function(7); //arguments
                         //paramters 形参，类型必须指定; arguments 实参

    //函数体中语句和表达式
    // 语句是执行指令，代码，没有返回值
    // 表达式是有计算结果，比如 5+6
    let x = { x[1] + 1 };

    // let x = {
    //     x+1;  // error：语句没有返回值
    // };

    //函数返回值，最后一个表达式或者return
    let x = five();

    //注释
    /*注释 */
    //文档注释，以后再讲

    //分支
    let number = 3;
    if number > 5 {
        println!("condition was true")
    }
    //如果多个if else，可以用match重构
    //if是个表达式，可以放在=右边
    let number = if true { 5 } else { 6 };

    // 循环
    loop {
        println!("gan!");
        break number * 2;
    }

    while number!=0 {
        println!("gan!");
        break number * 2;
    }

    let a = [1,2,3,4,5]
    for ele in a.iter() {
        println!(ele);//ele是引用
    }

    //range,rev
    for n in (1..4).rev() {
        println!(n);//不包含4
    }

    //所有权，管理计算机内存的方式
    // 1、垃圾收集
    // 2、显示分配释放
    // 3、rust采用第三种：内存通过一个所有权来管理，其中包含一组编译器在编译时检查的规则
    // 在程序运行时，所有权特性不会减慢程序的运行速度
    // 所有权是为了管理heap上得数据，跟踪代码哪些部分正在使用heap的哪些数据，最小化heap上的重复数据，清理heap上未使用的数据以避免空间不足
    // 所有权规则
    // 每个值都有一个变量，这个变量是该值的所有者
    // 每个值同时只能由一个所有者
    // 当所有者超出作用域的时候，这个值将会被删除

    {
        let mut s = String::from("hello");
        s.push_str("world!");
        let s2 = s; // 指针，长度，容量复制 move
        println!("{}" s);// 不能再使用s了，rust为了避免二次释放指针指向的数据
    }//离开作用域会调用drop

    // 你也许会将复制指针，长度，容量视为浅拷贝，但由于rust让s1失效了，所以我们用一个新的术语：移动（Move）
    // rust不会自动深copy，深拷贝可以使用clone，clone比较消耗资源，会复制堆上的数据
    // 标量有copy trait: bool,u32,char,f64,tuple里面都是copy的


}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    //paramters
    println!("another_function")
}

const MAX_POINT: u32 = 1000;
