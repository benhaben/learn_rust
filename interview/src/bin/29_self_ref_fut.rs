//! 压轴：自引用 Future
//!
//! 运行：`cargo run --bin 29_self_ref_fut`
//!
//! 看这段：
//! ```ignore
//! async fn hold() {
//!     let s = String::from("tick");
//!     use_ref(&s).await;
//! }
//! ```
//!
//! 编译器生成的状态机直觉上像：
//! ```text
//! struct Fut {
//!     s: String,
//!     inner: 某个借用了 s 的 Future,   // 里面有指向 s 的指针
//!     state: 进行到哪一步,
//! }
//! ```
//!
//! 第一次 poll 之前，`s` 还在栈上临时位置；第一次 poll 后 `s` 被放到状态机里，
//! 后续 `inner` 指向它。如果此时把整个 Fut **move 到别处**，内部指针悬空。
//! 这就是 Pin 存在的理由。
//!
//! 源码里不必给 s 写 Pin：钉的是整颗生成的 Future。hold().await 时运行时已经钉住。
//! 不要手写自引用结构，除非你愿意维护指针不变量（通常配合 Pin + 裸指针）。

async fn hold() {
    let s = String::from("tick");
    // s.len() 在 await 之前算完，这个例子其实不一定自引用。
    // 真正自引用是：await 的那个 Future 还握着 &s。
    let n = echo(&s).await;
    println!("await 之后仍能用 s = {s}, n = {n}");
}

async fn echo(s: &str) -> usize {
    tokio::task::yield_now().await; // 让出一次，状态机必须把 &str 存起来
    s.len()
}

#[tokio::main]
async fn main() {
    hold().await;

    // 等价直觉（不要当能编译的真代码）：
    // struct Fut { s: String, ptr: *const u8 }  // ptr 指向 s 的堆缓冲
}
