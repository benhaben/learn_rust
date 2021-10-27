use std::fmt::format;

// rust开发者经常被字符串困扰
// 1. rust报错多
// 2. 字符串本身复杂
// 3. utf-8

// 字符串是什么
// 1. byte的集合
// 2. 一些方法

// rust的核心语言层面，只有一个字符串类型：字符串切片str(或&str)
// 对存贮在其他地方，utf-8编码的字符串的引用
// - 字符串字面值：存贮在二进制文件中，也是字符串切片

// String类型
// -来自标准库和不是核心语言
// 可增值，可修改，可拥有
// utf-8编码

// rust标准库还有很多字符串类型：OsString，OsStr，CString，CStr
// string vs str ： 拥有或借用的变体
// 可存贮不同编码的文本或在内存中以不同的形式展现
// library crate 针对存贮字符串可提供更多的选项

// 很多vec<T>的操作可以用于String
// String::new
// to_string 可以用于实现了Display trait的对象
// String::from

// 更新String
// push_str:把一个字符串切边附加到string
// push 单个字符
// + 拼接字符串
// - 使用了fn add(self, s: &str)->String
// 标准库中的add使用了泛型
// 只能把&str添加到String
// 解引用强制转换（deref coercion）

fn main() {
    let s1 = String::from("xx");
    let s2 = String::from("xx");

    let s3 = s1 + &s2;
    // println!("{}", s1);//error，s1不能再用
    println!("{}", s2);
    println!("{}", s3);

    // 不会拿到参数所有权
    format!("{}-{}-{}", "1", "2", "3");

    // rust字符串不支持索引访问
    // 字符都是unicode标量值，索引不能保证能访问到合法的字符
    // 字节，标量值，字形簇 bytes，scalar value，grapheme cluster:https://www.zhihu.com/question/22132719
 
    let w = "啊是啊是大户";

    // 字节方式看待
    for b in w.bytes(){
        println!("----{}",b);
    }

    //标量值
    for b in w.chars(){
        println!("{}",b);
    }

    // 切割
    let s = &w[0..3];
    println!("xxxxxx-{}",s);

    //error:必须合理的切割
    // let s1 = &w[0..3];
    // println!("xxxxxx-{}",s1);

    // 好处：不用后期处理非asii编码
    // 坏处：不能索引，utf-8


}
