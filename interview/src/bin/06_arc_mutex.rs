//! 必考：Arc<Mutex<T>> 跨线程共享可变
//!
//! 运行：`cargo run --bin 06_arc_mutex`
//!
//! # 为什么是这两层
//!
//! - **Arc**：多个线程各自拥有一份“指向同一把锁的指针”，引用计数原子加减。
//! - **Mutex**：同一时刻只有一个线程能拿到里面的 T。
//!
//! `Rc<RefCell<T>>` 是单线程版；跨线程换成 `Arc<Mutex<T>>`。
//!
//! # spawn 为什么要 move
//!
//! `thread::spawn` 要求闭包 `FnOnce + Send + 'static`：
//! - 线程可能比当前函数活得更久，不能借栈上的局部变量（所以 `'static`）。
//! - 必须把用到的值**搬进**闭包（所以 `move`）。
//! - 值本身要能过线程边界（所以 `Send`）。
//!
//! 忘记 `Arc::clone`：第一个 spawn 把唯一的 Arc move 走，后面的循环编译失败。
//!
//! # poison
//!
//! 持锁线程 panic，锁被标记中毒。`lock()` 返回 `Err`。
//! 面试说：说明不变量可能坏了，生产上要定义是恢复还是退出。
//!
//! # 和 async
//!
//! `std::sync::MutexGuard` 不是 Send，不能跨 `.await`。见 `13_mutex_await`。

use std::sync::{Arc, Mutex};
use std::thread;

fn bump(n: usize) -> i32 {
    let hit = Arc::new(Mutex::new(0));

    // 必须先 collect JoinHandle，再 join。
    // 若在 map 里立刻 join，就变成串行，体现不出并行。
    let joins: Vec<_> = (0..n)
        .map(|_| {
            let hit = Arc::clone(&hit); // 计数 +1，原 hit 仍留给后面的迭代 / 主线程
            thread::spawn(move || {
                // lock() 失败 = 中毒。这里 unwrap 表示“中毒就让本线程也崩”。
                *hit.lock().unwrap() += 1;
            })
        })
        .collect();

    for j in joins {
        j.join().unwrap();
    }
    // 先把 Guard 里的值拷出来，再让 Guard Drop；不能把 lock() 当函数最后一行表达式。
    let total = *hit.lock().unwrap();
    total
}

fn main() {
    let n = 8;
    let total = bump(n);
    println!("{n} 个线程各 +1，结果 = {total}");
    assert_eq!(total, n as i32);
}
