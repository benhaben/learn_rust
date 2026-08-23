//! 压轴：Atomic 内存序
//!
//! 运行：`cargo run --bin 30_atomic_order`
//!
//! | 序 | 保证 | 典型用法 |
//! |---|---|---|
//! | Relaxed | 只有原子性，没有 happens-before | 计数器 |
//! | Release（写） / Acquire（读） | 配对后，写之前的内存操作对读之后可见 | 发布 flag + payload |
//! | AcqRel | 读改写同时具备两种 | fetch_add 当同步点 |
//! | SeqCst | 所有线程看到同一总序 | 很少需要，别当默认 |
//!
//! 陷阱：flag 用 Relaxed、数据用普通写，对面可能看到 flag 真、数据还是旧的。
//! x86 上很多错误代码“看起来总能过”，ARM 上翻车。
//!
//! 口条：先写数据（Relaxed / 普通），再 Release 发信号；对面 Acquire 看到真之后再读数据。

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

static READY: AtomicBool = AtomicBool::new(false);
static PX: AtomicU64 = AtomicU64::new(0);

fn publish(px: u64) {
    PX.store(px, Ordering::Relaxed);
    // Release：这之前对 PX 的写入，对随后 Acquire 到 true 的线程可见
    READY.store(true, Ordering::Release);
}

fn consume() -> Option<u64> {
    if READY.load(Ordering::Acquire) {
        Some(PX.load(Ordering::Relaxed))
    } else {
        None
    }
}

fn main() {
    let h = thread::spawn(|| {
        publish(101);
    });
    h.join().unwrap();

    let px = consume().expect("join 之后一定已经 publish");
    println!("Acquire 之后读到 px = {px}");
    assert_eq!(px, 101);
}
