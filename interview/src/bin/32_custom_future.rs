//! 压轴：自定义 Future / Waker
//!
//! 运行：`cargo run --bin 32_custom_future`
//!
//! Future 状态机协议：
//! 1. executor 调 `poll`
//! 2. 还没好：登记 **Waker**，返回 `Pending`
//! 3. IO / 另一线程就绪后调 `waker.wake()`
//! 4. executor 再 poll，直到 `Ready`
//!
//! 陷阱：
//! - 不存 Waker 就永远 Pending（没人叫你起来）
//! - 假唤醒必须能再次 Pending（不能假设 wake 一次就一定好了）
//! - poll 里不能阻塞
//!
//! 本例：另一个线程把 flag 置位并 wake。`select!` / `join!` 也是在组合 poll。

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

struct ReadyFlag {
    flag: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Future for ReadyFlag {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.flag.load(Ordering::Acquire) {
            return Poll::Ready(());
        }
        // 存下当前任务的 Waker。flag 线程稍后 wake。
        *self.waker.lock().unwrap() = Some(cx.waker().clone());
        // 再读一次，避免“刚存完 Waker、对面已经 set”的竞态
        if self.flag.load(Ordering::Acquire) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let flag = Arc::new(AtomicBool::new(false));
    let waker = Arc::new(Mutex::new(None));
    let fut = ReadyFlag {
        flag: Arc::clone(&flag),
        waker: Arc::clone(&waker),
    };

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        flag.store(true, Ordering::Release);
        if let Some(w) = waker.lock().unwrap().take() {
            w.wake(); // 通知 executor 再 poll
        }
    });

    fut.await;
    println!("自定义 Future 被 wake 之后变成 Ready");
}
