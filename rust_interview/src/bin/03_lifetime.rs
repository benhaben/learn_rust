//! 必考：Lifetime 省略 vs 手写
//!
//! 运行：`cargo run --bin 03_lifetime`
//!
//! # `'a` 是什么
//!
//! `'a` **不是**分配内存，也不是“这个值活 10 秒”。
//! 它是编译器的约束：**这个引用必须在这段时期内指向仍然有效的数据**。
//!
//! - 不能返回局部变量的引用：局部变量在函数结束时 Drop，引用会悬空。
//! - `T: 'static` **不是**“必须是全局常量”。它表示 `T` 内部不持有短寿命借用。
//!   `String`、`i32`、`&'static str` 都满足 `T: 'static`。
//!   `&local_string` 不满足，因为引用的目标会先死。
//!
//! # 省略规则（函数）
//!
//! 1. 每个输入引用各得到一个寿命参数。
//! 2. 只有一个输入寿命 → 输出引用用它。
//! 3. 有 `&self` / `&mut self` → 输出用 self 的寿命。
//! 4. 多个输入寿命、又没有 self → 必须手写，否则编译器不知道输出跟谁走。

/// 一个输入引用 → 输出跟它活一样久。等价于 `fn first<'a>(s: &'a str) -> &'a str`。
fn first(s: &str) -> &str {
    s
}

/// 两个输入：必须标同一个 `'a`，表示“返回值不会比 x、y 里较短的那个活得更久”。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 结构体里只要存了引用，就必须带寿命参数。
/// 意思：`Holder` 不能比它里面的 `name` 活得更久。
struct Holder<'a> {
    name: &'a str,
}

// fn dangling() -> &str {
//     let s = String::from("x");
//     &s // s 在这里被 Drop，返回的引用会悬空 → 编译失败
// }

fn main() {
    let owned = String::from("hello");
    println!("first: {}", first(&owned));

    let a = "ab";
    let b = "abcd";
    println!("longest: {}", longest(a, b));

    let h = Holder { name: "tick" };
    println!("Holder 借着字面量（'static）: {}", h.name);

    // T: 'static 的常见误解：
    let ok: Box<dyn Send> = Box::new(String::from("owned")); // String: 'static
    let _ = ok;
    // let local = String::from("tmp");
    // let bad: Box<dyn Send> = Box::new(&local); // &local 不够 'static
}
