//! 压轴：pyo3 与 GIL（口条 + 可编译的锁顺序示意）
//!
//! 运行：`cargo run --bin 34_pyo3_gil`
//!
//! 真项目才会加 `pyo3` 依赖。这里用 Rust 锁模拟同一条死锁路径，方便你自己编译。
//!
//! # 口条
//!
//! - 研究用 Python，热路径用 Rust。导出纯函数或批量数组，不要每 tick 过一次 Python。
//! - **过 GIL 时不要持着 Rust Mutex。**
//! - 典型死锁：
//!   1. Python 持 GIL，调进 Rust，Rust 再拿 `books` 锁
//!   2. 另一条路径：Rust 持着 `books` 锁，再调 `py.callback()` 去拿 GIL
//!   两边互相等。
//!
//! 正确：锁内只 clone / 拷贝 Rust 数据 → 出锁 → 再碰 Python 对象。

use std::sync::{Arc, Mutex};
use std::thread;

struct Book {
    last: i64,
}

/// 错误顺序（示意）：持锁时去“回调 Python”。
/// 这里用第二把锁模拟 GIL。不要在生产里这么写。
#[allow(dead_code)]
fn _deadlock_shape(books: &Mutex<Book>, gil: &Mutex<()>) {
    let _book = books.lock().unwrap();
    let _py = gil.lock().unwrap(); // 持着 books 再拿 GIL
}

/// 正确：先出 Rust 锁，再碰“Python”。
fn on_tick(books: &Mutex<Book>, gil: &Mutex<()>) -> i64 {
    let snap = { books.lock().unwrap().last };
    let _py = gil.lock().unwrap();
    // 这里才构造 PyObject / 调回调
    snap
}

fn main() {
    let books = Arc::new(Mutex::new(Book { last: 101 }));
    let gil = Arc::new(Mutex::new(()));

    let b = Arc::clone(&books);
    let g = Arc::clone(&gil);
    let h = thread::spawn(move || {
        let n = on_tick(&b, &g);
        println!("线程里出锁后再拿 GIL, last = {n}");
    });

    {
        books.lock().unwrap().last = 102;
    }
    h.join().unwrap();
    println!("主线程 last = {}", books.lock().unwrap().last);
}
