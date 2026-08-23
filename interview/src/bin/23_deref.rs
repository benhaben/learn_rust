//! 进阶：Deref 强制转换
//!
//! 运行：`cargo run --bin 23_deref`
//!
//! `Deref` 让智能指针用起来像里面的 T：
//! - `&String` → `&str`
//! - `&Vec<T>` → `&[T]`
//! - `&Rc<T>` / `&Box<T>` → `&T`
//!
//! 这是**沿引用链**的强制，不会凭空把 `&str` 变成 `String`。
//! 要可变目标再 impl `DerefMut`。
//!
//! 方法解析也会自动解引用：`s.len()` 其实是 `str::len(&s)`。
//! 自定义 Deref 别玩花的，否则方法解析会变得很难读。

use std::ops::Deref;

fn takes_str(s: &str) {
    println!("吃到 &str: {s}");
}

fn main() {
    let s = String::from("hi");
    takes_str(&s); // &String → &str

    let b = Box::new(1);
    assert_eq!(*b.deref(), 1);
    assert_eq!(*b, 1); // *Box<T> 也走 Deref

    let v = vec![1, 2, 3];
    let slice: &[i32] = &v; // &Vec → &[T]
    println!("切片 = {slice:?}");
}
