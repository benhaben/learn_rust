//! 进阶：关联类型 vs 泛型参数
//!
//! 运行：`cargo run --bin 16_assoc_type`
//!
//! # 两种都是「trait 里再出现一个类型」
//!
//! 都是在说：实现这个 trait 时，还要指定一个类型（节点是什么、从什么转过来）。
//! 差别是：**一份 `impl` 能不能对同一个实现者选好几种。**
//!
//! # 关联类型：一个实现者只许选定一种
//!
//! ```ignore
//! trait Graph {
//!     type Node;                    // 由 impl 选定，不是调用方传入
//!     fn start(&self) -> Self::Node;
//! }
//! impl Graph for () {
//!     type Node = u32;              // () 当图用时，节点只能是 u32
//! }
//! ```
//!
//! `()` 是 unit 类型（空类型），不是给括号做运算符重载。更顺眼可写成 `struct EmptyGraph;`。
//! 不能再写一份 `impl Graph for () { type Node = String; }`：
//! 同一个 `Self` 对同一个 trait 只能有一份 impl。
//! 所以看见 `().start()`，编译器已经知道返回值是 `u32`，不用写涡轮鱼（`::<>`）。
//! `From<T>` 变的是输入，返回仍是 `String`，不是「泛型才能返回不同类型」。
//!
//! `Iterator::Item` 也是这样：`vec.iter()` 的元素类型定死了，不会又是 `i32` 又是 `String`。
//!
//! # 泛型参数：同一个类型可以对很多 T 各 impl 一次
//!
//! ```ignore
//! trait From<T> { fn from(value: T) -> Self; }
//! impl From<&str> for String { ... }
//! impl From<char>  for String { ... }
//! ```
//!
//! `String` 既可以从 `&str` 来，也可以从 `char` 来。这是**多对多**。
//! 若把 Graph 写成 `trait Graph<Node>`，就可以同时
//! `impl Graph<u32> for ()` 和 `impl Graph<String> for ()`。
//! 那时 `().start()` 就含糊了：哪一份 impl？必须写成 `Graph::<u32>::start(&())`。
//!
//! # 怎么选
//!
//! | 关系 | 用谁 | 例子 |
//! |---|---|---|
//! | 这个类型当这个 trait 用时，那一项只有一种 | 关联类型 | `Iterator::Item`、本文件的 `Graph::Node` |
//! | 同一个类型要对多种输入各成立一次 | 泛型参数 | `From<T>`、`PartialEq<Rhs>` |
//!
//! 口条：一种输出用关联类型；同一 trait 要对多种输入成立，用泛型参数。

trait Graph {
    /// 这个图的节点类型。由 impl 选定，每个实现者只有一种。
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
    // () 的 Graph::Node 已经定死是 u32，推断得出 n: u32。
    // 若 Graph 是 Graph<Node>，这里就要写 Graph::<u32>::start(&())。
    let n = ().start();
    println!("Graph::Node = {n}");

    // From<T>：同一个 String，两份 impl，两种输入。
    let a = String::from("hi");
    let b = String::from('x');
    println!("From<&str> = {a}, From<char> = {b}");
}
