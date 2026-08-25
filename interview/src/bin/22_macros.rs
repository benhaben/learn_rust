//! 进阶：宏（声明宏 / 过程宏）
//!
//! 运行：`cargo run --bin 22_macros`
//!
//! - **声明宏** `macro_rules!`：模式匹配 token，编译期展开。`vec!`、`println!`。
//! - **过程宏**：另一段 Rust 程序吃 token stream。三种：
//!   `#[derive(Debug)]`、属性宏 `#[tokio::main]`、函数式 `sqlx::query!`。
//!
//! 宏不是函数：没有类型、卫生性（不会意外捕获外部同名变量）。
//! 能用泛型 / 函数就别上宏。调试：`cargo expand`。
//!
//! 面试能手写一个 `vec!` 风格即可。

macro_rules! myvec {
    // 空
    () => {
        Vec::new()
    };
    // 不是正则：按语法抓表达式。$() 重复，:expr 是一段代码，+ 至少一个，$(,)? 允许末尾逗号。
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $( v.push($x); )+
        v
    }};
}

fn main() {
    let empty: Vec<i32> = myvec![];
    let v = myvec![1, 2, 3];
    let w = myvec![10, 20,];
    println!("empty={empty:?} v={v:?} w={w:?}");
}
