use std::{error::Error, fs::File, io::{self, ErrorKind, Read}, num::FpCategory};

// rust的可靠性：错误处理
// -大部分情况下：在编译时提示错误，并处理
// 错误的分类：
// -可以恢复：例如文件没有找到，可以再次尝试
// -不可以恢复
// bug，例如索引超出范围
// Rust没有异常
// -可恢复：Result<T,E>
// 不可恢复：panic!

// panic!宏执行
// 打印错误
// 展开unwind 或者 abort、清理调用栈（stack）
// 退出程序

// 展开调用栈，rust沿着调用栈往回走，清理每个函数中的数据，工作量大，默认
// 终止，不清理，OS清理内存
// [profile.release]
// panic = 'abort'

fn main() {
    let f = File::open("xx.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("xx.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };

    // unwrap：match表达式的一个快捷方法
    // expect：可以指定错误信息
    let f = File::open("xx1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("xx1.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file:{:?}", error);
        }
    });
}

// 传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//?运算符，传播错误的简化方式
fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ?与from函数
// Trait std::convert::From 上的from函数：用于错误之间的转换
// 被?所应用的错误，会隐式的贝from函数处理
// 当?调用from函数时：它所接受的错误类型会被转化为当前函数返回类型所定义的错误类型
// 用于：针对不同错误的原因，返回同一种错误类型：只要每个错误类型实现了转换为所返回的错误类型的from函数

// ?运算符与main函数
// main函数的返回类型是()
// main函数的返回类型也可以是：Result<T,E>
// Box<dyn Error>是trait对象：简单理解为任何可能的错误类型

fn main1() ->Result<(),Box<dyn Error>>{
    let f = File::open("hello.txt")?;
    Ok(())
}