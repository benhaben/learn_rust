use std::net::IpAddr;

// 总体原则
// 在定义一个可能失败的函数时，优先考虑返回result
// 否则就是panic!

// panic!
// 1. 编写例子，原型代码，测试：unwarp，expect能制造panic!
// 2. 测试代码，unwarp，expect
// 3. 你可以确定Result就是Ok，那你可以使用unwrap，你比编译器掌握更多信息

fn main() {
    //一定不会panic，可以使用unwrap
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

// 4. 当你的代码对值进行操作，首先应该验证这些值，不符合就panic
// 4. 调用你的代码，传入无意义的值，可以使用panic!警告，方便用户解决，
// 5. 调用外部不可控的代码，返回非法状态，你无法修复：panic!
// 6. 如果时报是可预期的：Result

// 实战
// 为验证创建自定义类型，把验证逻辑放在类型里面

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("你输入的很不靠谱！ ");
        }

        Guess { value }
    }

    //getter，私有的，保证必须使用new去创建
    pub fn value(&self) -> i32 {
        self.value
    }
}
