//! 必考：MutexGuard 不能跨 .await
//!
//! 运行：`cargo run --bin 13_mutex_await`
//!
//! # 为什么
//!
//! `.await` 时当前任务可能被挂起，恢复时可能在**另一个 worker 线程**上。
//! `std::sync::MutexGuard` 不是 `Send`（它内部认为锁和线程绑定），
//! 所以“持着 Guard 再 await”编译失败。
//!
//! # 怎么写
//!
//! 1. **优先**：缩小临界区，拷出数据，再 await（本文件的 `right`）。
//! 2. 临界区本身必须跨 await（要在锁里读网络）才用 `tokio::sync::Mutex`。
//! 3. 能不用异步锁就不用：异步锁更重，也更容易在 `.await` 里拿着锁睡着。
//!
//! 量化事故：持锁做 CPU、serde、或回调 Python。

async fn right(m: &std::sync::Mutex<i32>) {
    // 花括号结束 → Guard Drop → 解锁，然后才 await
    let n = { *m.lock().unwrap() };
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    println!("先拷出来再 await, n = {n}");
}

#[allow(dead_code)]
async fn _wrong(m: &std::sync::Mutex<i32>) {
    let _g = m.lock().unwrap();
    // 打开下一行会编译失败：`MutexGuard` cannot be sent between threads safely
    // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
}

#[tokio::main]
async fn main() {
    let m = std::sync::Mutex::new(7);
    right(&m).await;
}
