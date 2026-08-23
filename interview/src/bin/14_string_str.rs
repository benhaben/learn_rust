//! 必考：String / &str / Vec / &[T]
//!
//! 运行：`cargo run --bin 14_string_str`
//!
//! # 拥有 vs 借用
//!
//! | 拥有（可改、可存活任意久） | 借用（不分配、跟 owner 走） |
//! |---|---|
//! | `String` | `&str` |
//! | `Vec<T>` | `&[T]` |
//!
//! `&str` / `&[T]` 是**胖指针**：`(指针, 长度)`。不拥有数据。
//!
//! API 原则：参数能写 `&str` 就别写 `String`（调用方可以传字面量，也可以 `&String`，靠 Deref）。
//! 只有你要**存下来或返回新数据**才给 `String`。
//!
//! # UTF-8
//!
//! `String` 按字节索引。`s[0]` 对多字节字符可能切在码点中间 → panic。
//! 用 `.chars()` / `.get()` / 字节范围且保证落在字符边界。

fn title(s: &str) -> String {
    let mut t = s.trim().to_string();
    // 演示：只处理 ASCII 首字节。中文请用 chars。
    if let Some(c) = t.get_mut(0..1) {
        c.make_ascii_uppercase();
    }
    t
}

fn first(xs: &[i32]) -> Option<i32> {
    // copied：&i32 → i32，因为 i32: Copy。clone 也对，但 copied 更准确。
    xs.first().copied()
}

fn main() {
    let owned = String::from("  hello");
    // &String 会 Deref coerce 成 &str，所以 title(&owned) 可以
    println!("title = {}", title(&owned));
    println!("字面量也能传: {}", title("world"));

    println!("first = {:?}", first(&[9, 8, 7]));

    let zh = String::from("你好");
    println!("中文字节数 = {}, chars = {:?}", zh.len(), zh.chars().collect::<Vec<_>>());
    // println!("{}", &zh[0..1]); // 可能 panic：切在 UTF-8 中间
}
