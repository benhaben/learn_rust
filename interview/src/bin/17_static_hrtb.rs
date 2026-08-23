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
//! 普通 `fn foo(s: &str)` 的寿命由调用处推断一次。
//! `for<'a>` 表示：**任意**寿命的 `&'a str` 都能传进去。
//! 闭包如果把输入引用存到自己捕获的结构里，就不能对所有 `'a` 成立，HRTB 会拒。

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
