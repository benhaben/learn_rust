//! 进阶：关联类型 vs 泛型参数
//!
//! 运行：`cargo run --bin 16_assoc_type`
//!
//! - **关联类型**（`type Item`）：每个 `impl Trait for Type` **只能选定一种**。
//!   调用方少写涡轮鱼。`Iterator::Item`、`Add::Output` 都是这种“一种输出”。
//! - **泛型参数**（`From<T>`）：同一个类型可以对很多 T 各 impl 一次。
//!   这是多对多关系。
//!
//! 口条：一种输出用关联类型；同一 trait 要对多种输入成立，用泛型参数。

trait Graph {
    type Node;
    fn start(&self) -> Self::Node;
}

impl Graph for () {
    type Node = u32;
    fn start(&self) -> u32 {
        0
    }
}

fn main() {
    let n = ().start(); // 不用写 Graph<Node = u32>，关联类型已经定死
    println!("Graph::Node = {n}");

    // From<T> 是泛型参数：一个 String 可以从很多类型来
    let a = String::from("hi");
    let b = String::from('x');
    println!("From<&str> = {a}, From<char> = {b}");
}
