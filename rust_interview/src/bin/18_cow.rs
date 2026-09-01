//! 进阶：Cow——能借就借，要改才拷贝
//!
//! 运行：`cargo run --bin 18_cow`
//!
//! # 要解决什么
//!
//! 函数吃一段 `&str`，有时原样还给你（没空格），有时必须改（空格换成 `_`）。
//! 若一律 `to_string()`，没改的那次也白分配一次堆。
//! 若返回 `&str`，改过的那次新 `String` 在函数结束就没了，借不出去。
//!
//! `Cow`（Clone-on-Write，写时克隆）是一个枚举，两种都能装：
//! - `Borrowed`：里面是借用，不分配
//! - `Owned`：里面是自己的 `String`，改过 / 拷过才走这条
//!
//! # `use std::borrow::Cow` 是干什么的
//!
//! 标准库里这个类型的完整路径是 `std::borrow::Cow`。
//! `use` 只是引进当前文件，下面才能写 `Cow`、`Cow::Borrowed`，
//! 而不用每次写 `std::borrow::Cow::Borrowed`。
//! 它不创建对象，也不启动写时克隆；只是导入名字。
//!
//! `borrow` 模块放的是「借用 / 拥有」相关的类型（`Cow`、`ToOwned`）。
//! `Cow<'a, str>` 的 `'a` 是借用那一支能活多久；`str` 是借的那种子类型
//! （对应的拥有类型是 `String`，由 `ToOwned` 规定）。
//!
//! # 本文件在做什么
//!
//! `norm`：没有空格 → `Borrowed`，调用方看到的还是原来那串字；
//! 有空格 → `replace` 得到新 `String`，装进 `Owned`。
//! 读多改少（规范化路径、配置、反序列化）合适。
//! 热路径里其实更常见「直接 `to_string()`」，别为了 Cow 而 Cow。
//!
//! `into_owned()`：不管现在是借还是拥有，都变成 `String`。
//! 本来是借的，这时才会 clone 一次。
//!
//! `replace` 那行没有 `.clone()`，是直接造新串再装进 Owned。
//! 名字里的克隆发生在 `to_mut` / `into_owned` 遇上 Borrowed 的时候。
//!
//! 口条：可能改、也可能不改，用 Cow。没改就零分配；要改才买堆。

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
