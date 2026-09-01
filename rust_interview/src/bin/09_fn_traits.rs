//! 必考：FnOnce / FnMut / Fn
//!
//! 运行：`cargo run --bin 09_fn_traits`
//!
//! 闭包按**怎么捕获环境**自动实现这三者（它们是子 trait：`Fn: FnMut: FnOnce`）。
//!
//! | trait | 捕获 | 能调用几次 | 例子 |
//! |---|---|---|---|
//! | FnOnce | 拿走所有权（move 出环境） | 一次 | `drop(s)`、`thread::spawn` |
//! | FnMut | 可变借环境 | 多次，会改捕获的变量 | `sort_by` 里改计数器 |
//! | Fn | 共享借环境，或不捕获 | 多次，不改环境 | `\|x\| x + 1` |
//!
//! 注意：写了 `move` 的闭包**仍可能是 Fn**——如果捕获的是 Copy 值，每次调用只是复制。
//! `Box<dyn FnOnce()>` 只能 `call_once` 一次，之后盒子被消耗。

fn call_once<F: FnOnce()>(f: F) {
    f();
    // f(); // FnOnce 只能调一次
}

fn call_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn call<F: Fn()>(f: F) {
    f();
    f();
}

fn main() {
    // FnOnce：闭包把 String 的所有权吃掉
    let s = String::from("x");
    call_once(move || {
        println!("FnOnce 吃掉了 {s}");
        drop(s);
    });
    // println!("{s}"); // 已 move 进闭包

    // FnMut：闭包可变借用 n
    let mut n = 0;
    call_mut(|| {
        n += 1;
    });
    println!("FnMut 之后 n = {n}");

    // Fn：什么都没改
    call(|| println!("Fn 可以反复调用"));

    // spawn 要 FnOnce + Send + 'static，所以几乎总是 `move || ...`
}
