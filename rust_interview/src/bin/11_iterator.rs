//! 必考：Iterator / 适配器 / collect
//!
//! 运行：`cargo run --bin 11_iterator`
//!
//! # 三件套（所有权）
//!
//! | 写法 | 方法 | 元素类型 | 是否拿走 Vec |
//! |---|---|---|---|
//! | `for x in v` | `into_iter()` | `T` | 是 |
//! | `for x in &v` | `iter()` | `&T` | 否 |
//! | `for x in &mut v` | `iter_mut()` | `&mut T` | 否 |
//!
//! # 惰性
//!
//! `map` / `filter` **不跑**。遇到 `collect` / `sum` / `for` / `fold` 才开始拉元素。
//! 所以“map 完还要 collect”：你只是搭了一条管道，collect 才是消费器。
//!
//! `filter` 的闭包拿到的是 `&Item`。`copied()` 把 `&i32` 变成 `i32`（因为 Copy）。
//!
//! 单态化后热路径和手写循环一个级别。能 `fold` 就别先 `collect` 成中间 Vec。

fn evens(xs: &[i32]) -> Vec<i32> {
    xs.iter()
        .copied() // &i32 → i32（Copy）
        .filter(|x| x % 2 == 0) // 这里 x 是 &i32，因为 filter 借 Item
        .collect()
}

fn sum_sq(xs: &[i32]) -> i32 {
    xs.iter().map(|x| x * x).sum()
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("evens = {:?}", evens(&v));
    println!("sum_sq = {}", sum_sq(&v));

    // into_iter 拿走所有权
    let sum: i32 = v.into_iter().sum();
    println!("into_iter 之后原 Vec 不能再用, sum = {sum}");
    // println!("{v:?}"); // move

    // turbo-fish：collect 目标类型不明确时要写
    let ids = ["1", "2"];
    let parsed = ids.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    println!("turbo-fish collect = {parsed:?}");
}
