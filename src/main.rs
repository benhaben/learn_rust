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
            Err(_) => continue, // TODO: 错误是程序退出还是怎么地
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
        break;
    }

    while number != 0 {
        println!("gan!");
        break;
    }

    let a = [1, 2, 3, 4, 5];
    for ele in a.iter() {
        println!("{}", ele); //ele是引用
    }

    //range,rev
    for n in (1..4).rev() {
        println!("{}", n); //不包含4
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
                    //println!("{}", s);// 不能再使用s了，rust为了避免二次释放指针指向的数据
    } //离开作用域会调用drop

    // 你也许会将复制指针，长度，容量视为浅拷贝，但由于rust让s1失效了，所以我们用一个新的术语：移动（Move）
    // rust不会自动深copy，深拷贝可以使用clone，clone比较消耗资源，会复制堆上的数据
    // 标量有copy trait: bool,u32,char,f64,tuple里面都是copy的

    // 所有权和函数，所有权和返回值，一个变量的所有权总是遵循相同的模式
    // 把一个值付给其他值就会移动
    // 当一个包含heap数据的变量离开作用域时，他的值就会被drop函数清理，除非数据的所有权移动到另一个变量上了

    {
        // 变量传给函数后怎么再拿到所有权呢
        let s1 = String::from("he");
        let (s2, len) = cal_len(s1);
        //println!("{}",s1); s1已经被move了
        println!("{}", s2); // ok
    }

    // 上面这种办法比较笨，介绍引用，解引用*，指向指针的指针。
    // 把引用作为函数变量叫借用，借用的变量不可以改变，需要加mut
    {
        let s1 = String::from("he");
        cal_len1(&s1);
        println!("{}", s1); // ok
    }

    //可变引用
    //限制：在特定作用域内，对某一块数据，只能由一个可变引用，这样做的好处是可以防止数据竞争
    //以下三种行为会发生数据竞争：
    // - 两个或者多个指针同时访问同一个数据
    // - 至少有一个指针用于写入数据
    // - 没有使用任何机制来同步对数据的访问
    // rust在编译的时候就防止了这三种情况
    // 但rust可以通过创建新的作用域，来允许非同时的创建多个可变引用
    //不可以同时拥有一个可变引用和不可变引用
    {
        let mut s = String::from("he");
        //但rust可以通过创建新的作用域，来允许非同时的创建多个可变引用
        {
            let s1 = &mut s;
        }
        let s2 = &mut s;
        // let s2 = &mut s; //ERROR: second mutable borrow occurs here
        // println!("{}", s1); // ok

        cal_len2(&mut s);
        println!("{}", s); // ok
    }
    // 空指针错误，悬空引用
    // rust编译器可以保证引用永远不是悬空的
    // let r = dangle();

    // 引用规则
    // 在任意给定时刻，只能满足下列条件之一：1. 一个可变的引用 2. 任意数量不可变的引用
    // 引用必须一直有效

    // 切片
    // 解决的问题：
    {
        let mut s = String::from("hhhe xxx");
        let world_index = first_world(&s);
        // 如果此时s.clear, 下面wordIndex就没用了，所以可能导致bug，rust对这种代码使用切片
        println!("{}", world_index);
    }
    // 切边定义: 指向字符串中一部分的引用
    // [开始索引..结束索引]
    {
        let mut s = String::from("hello world");
        let h = &s[..5];
        let w = &s[6..];
        let all = &[..];
        let world_index = first_world1(&s);
        //s.clear(); //error: mutable borrow occurs here
        println!("{}", world_index);
    }

    // 字符串字面值就是切片
    // 将字符串切片作为参数传递
    // 有经验的rust程序员会这样写 fn first_world(s:&str)->&str
    // 这样就可以直接被字符串字面值当参数，字符串类型也可以，会使api更通用
    {
        let mut s = String::from("hello world");
        let world_index = first_world2(&s[..]); //切片是不可变的
                                                // let world_index = first_world2("hello world");
                                                //s.clear(); //error: mutable borrow occurs here
        println!("{}", world_index);
    }
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_world1(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_world2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 编译不过
// fn dangle()-> &String{
//     let s = String::from("xx");
//     &s
// }

fn cal_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn cal_len1(s: &String) -> usize {
    let len = s.len();
    return len;
}

fn cal_len2(s: &mut String) -> usize {
    let len = s.len();
    return len;
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    //paramters
    println!("another_function")
}

const MAX_POINT: u32 = 1000;

// struct
struct User {
    username: String,
    age: u32,
}

fn assign_struct() {
    let user1 = User {
        username: String::from("xxxx"),
        age: 22,
    };
}

fn struct_as_biaodashi() -> User {
    User {
        username: String::from("xxxx"),
        age: 22,
    }
}

fn struct_jianxie(username: String) {
    User { username, age: 22 };
}

// struct 更新语法
fn struct_update(user1: User) {
    let user2 = User {
        username: String::from("xxxx"),
        ..user1
    };
}

// tuple struct
// tuple struct 整体有个名，但里面元素没有名字
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit like struct 没有任何字段，和空元祖()}类似
// 适用于需要在某个类型上实现某个trait，但是里面有没有想要存贮的数据

//struct数据的所有权，可以放引用，但需要用到生命周期
struct UserRef {
    // username: &str, //error 没有指定生命周期
}

#[derive(Debug)]
struct Rectangele {
    width: u32,
    length: u32,
}
fn debug_print(){
    let rect = Rectangele{
        width:33,
        length:22,
    };
    println!("{:#?}", rect);
}

