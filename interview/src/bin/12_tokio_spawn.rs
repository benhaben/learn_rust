//! 必考：tokio::spawn / Send + 'static
//!
//! 运行：`cargo run --bin 12_tokio_spawn`
//!
//! # async fn 是什么
//!
//! `async fn go(...) -> T` **不会立刻执行**。它只是返回一个 `impl Future<Output = T>`。
//! 有人 `.await` 或 executor `poll` 它，里面的代码才跑。
//!
//! # spawn 的约束
//!
//! `tokio::spawn(fut)` 把 Future 丢到运行时线程池：
//! - 任务可能在别的线程上 poll → Future 必须 **Send**
//! - 任务可能比当前栈帧活得久 → 必须 **'static**（不能借局部变量）
//!
//! `Rc` / `RefCell` / 没加 `move` 的 `&local` 都进不去。
//!
//! # 别堵 runtime
//!
//! worker 线程在跑你的 Future。里面写 `for _ in 0..1e9 {}` 或同步读盘，
//! 整个 runtime 的其他任务都会饿死。CPU 重活用 `spawn_blocking`（或独立线程）。

async fn go(n: i32) -> i32 {
    n + 1
}

fn heavy_cpu() -> u64 {
    (0..10_000).sum()
}

#[tokio::main]
async fn main() {
    // spawn 返回 JoinHandle，本身也是 Future，要 .await 才能拿到结果
    let h = tokio::spawn(async { go(1).await });
    let v = h.await.expect("任务 panic 了");
    println!("spawn 结果 = {v}");

    let n = tokio::task::spawn_blocking(heavy_cpu)
        .await
        .expect("blocking 任务 panic");
    println!("spawn_blocking CPU 结果 = {n}");

    // 下面这种借局部变量过不了 'static：
    // let local = String::from("no");
    // tokio::spawn(async { println!("{local}") }); // local 被借，任务可能活更久
}
