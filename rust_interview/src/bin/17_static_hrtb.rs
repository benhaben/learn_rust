//! 进阶：'static / HRTB（for<'a>）
//!
//! 运行：`cargo run --bin 17_static_hrtb`
//!
//! # T: 'static
//!
//! **不是**“这个值必须是全局常量 / 必须活到进程结束”。
//! 意思是：T **里面没有指向短寿命数据的引用**。拥有自己数据的 `String` 是 `'static`。
//! `&local` 不是，因为 local 先死。
//!
//! `spawn` 要 `'static`：任务可能在调用者返回之后还在跑，不能借调用者的栈。
//!
//! # HRTB：`for<'a> Fn(&'a str)`
//!
//! HRTB = Higher-Ranked Trait Bounds。`for` = 对所有寿命。
//! 约束的是闭包入参那一次借用能有多短：多短都行。
//! 主要为了：本函数栈上刚造的 `local`，能把 `&local` 传给 `f`，且 `f` 不能把引用藏起来。
//! `Fn(&str)` 在这种位置通常等于写出 `for<'a>`；业务里很少手写。
//!
//! `fn apply<'a, F: Fn(&'a str)>` 则是调用 apply 时定死一个 `'a`，罩不住里面的新局部变量。

fn spawnable<T: Send + 'static>(t: T) {
    std::thread::spawn(move || {
        let _ = t;
    })
    .join()
    .unwrap();
}

fn apply<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    // 两次调用传入的字面量寿命可以想成不同的 'a，F 都必须接受
    f("hi") + f("hello")
}

fn main() {
    spawnable(String::from("ok")); // String 拥有数据 → 'static

    // let s = String::from("x");
    // spawnable(&s); // &s 的寿命跟 s 走，不是 'static

    let n = apply(|s| s.len());
    println!("HRTB 闭包结果 = {n}");
}
