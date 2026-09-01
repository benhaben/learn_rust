//! 压轴：自定义 Future / Waker
//!
//! 运行：`cargo run --bin 32_custom_future`
//!
//! # 意图
//!
//! 演示「自己实现一个 Future」时必须遵守的协议，不是业务里要手写 Future。
//! 日常用 `tokio::time::sleep` 即可。本例假装：另一条 OS 线程 30ms 后把活干完，
//! 再通知 tokio「可以继续 poll 了」。
//!
//! # wake 不是唤醒那条 spawn 出来的线程
//!
//! `thread::spawn` 的线程自己跑完就退出，没人睡在那里等 wake。
//! `w.wake()` 叫醒的是 **tokio 里那个在 await 的任务**：请 executor 再 poll 一次。
//! 执行器 = `#[tokio::main]` 的 worker，不是 `thread::spawn` 那条。
//!
//! # 执行顺序（常见路径）
//!
//! ```text
//! 1. 造好 flag / waker / ReadyFlag（都还没 poll）
//! 2. thread::spawn：后台线程开始 sleep 30ms（和 tokio 并行）
//! 3. fut.await：tokio 第一次 poll
//!       旗是 false → 把「怎么叫醒我」存进 waker → Pending
//!       当前任务从队列拿掉，worker 去干别的（不阻塞这条 OS 线程死等）
//! 4. 后台线程睡醒：flag=true，取出 Waker，wake()
//! 5. tokio 把任务排回去，第二次 poll
//!       旗已 true → Ready，await 结束，打印
//! ```
//!
//! 也可能线程先跑完（机器极快）：第一次 poll 就看见 true，直接 Ready，wake 那步可有可无。
//! 所以 poll 里要先看旗，存 Waker 后再看一次（避免「对面已 set、你却 Pending 到死」）。
//!
//! poll 里不能 thread::sleep。select! / join! 也是在组合别人的 poll。

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

/// 等「旗子变 true」的 Future。两块共享状态给后台线程用。
struct ReadyFlag {
    flag: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Future for ReadyFlag {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // 已经好了（包括线程跑在第一次 poll 之前）：直接结束，不必 wake。
        if self.flag.load(Ordering::Acquire) {
            return Poll::Ready(());
        }
        // 还没好：登记「以后请叫 tokio 再 poll 我」。cx.waker() 是执行器给的。
        *self.waker.lock().unwrap() = Some(cx.waker().clone());
        // 刚登记完，线程可能已经 set 了：再看一眼，免得永远 Pending。
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

    // 先启动后台线程。它不跑 Future，只负责：等一会儿、立旗、wake。
    // 此时 tokio 还没 poll，waker 里常常还是 None；30ms 后一般已经 poll 过、里面有 Waker。
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        flag.store(true, Ordering::Release);
        if let Some(w) = waker.lock().unwrap().take() {
            // 通知 tokio 再 poll fut，不是唤醒本线程（本线程随后就结束）。
            w.wake();
        }
    });

    // 这里才第一次（以及 wake 之后再）poll。卡住的是「这个 async 任务」，不是整个进程。
    fut.await;
    println!("自定义 Future 被 wake 之后变成 Ready");
}
