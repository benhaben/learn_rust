//! 进阶：Cow<'a, T>（Clone-on-Write）
//!
//! 运行：`cargo run --bin 18_cow`
//!
//! `Cow<str>` 是枚举：
//! - `Borrowed(&'a str)`：没改，零分配
//! - `Owned(String)`：必须改时才克隆 / 分配
//!
//! API 语义是“**可能**分配”。读多改少（规范化路径、配置、反序列化）很合适。
//! 热路径里无脑 `to_string()` 比“不会 Cow”更常见。
//!
//! `into_owned()` 总会得到拥有值：本来是借的就会 clone 一次。

use std::borrow::Cow;

fn norm(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        Cow::Borrowed(s)
    }
}

fn main() {
    match norm("a_b") {
        Cow::Borrowed(v) => println!("没分配: {v}"),
        Cow::Owned(_) => panic!("不该分配"),
    }
    match norm("a b") {
        Cow::Owned(v) => println!("才分配: {v}"),
        Cow::Borrowed(_) => panic!("该分配"),
    }
    assert_eq!(norm("a_b"), "a_b");
    assert_eq!(norm("a b"), "a_b");
}
