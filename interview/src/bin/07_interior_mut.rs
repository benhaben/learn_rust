//! 必考：内部可变性 Cell / RefCell
//!
//! 运行：`cargo run --bin 07_interior_mut`
//!
//! # 问题
//!
//! 默认规则：有 `&T` 就不能改 T。但有时 API 必须是 `&self`
//!（例如图节点互相指、observer），内部又要改计数 / 缓存。
//!
//! **内部可变性**：外面看着是共享借，里面用 UnsafeCell 合法地改。
//! 这是把“编译期借检查”挪到类型 / 运行期。
//!
//! | | Cell | RefCell | Mutex |
//! |---|---|---|---|
//! | 线程 | 单线程 | 单线程 | 多线程 |
//! | 检查 | 无（只适合 Copy） | 运行期借计数，冲突 panic | 运行期锁 |
//! | 典型 | 计数器 | 图、缓存 | 跨线程共享可变 |
//!
//! 都不是 Sync（Mutex 除外）。逻辑错误用 RefCell 是在推迟爆炸。

use std::cell::{Cell, RefCell};

struct Counter {
    n: Cell<i32>,
}

impl Counter {
    /// `&self` 也能改：Cell::set 靠内部可变，不需要 `&mut self`。
    fn inc(&self) {
        self.n.set(self.n.get() + 1);
    }
}

fn main() {
    let c = Counter { n: Cell::new(0) };
    c.inc();
    c.inc();
    println!("Cell 计数 = {}", c.n.get());

    // RefCell：运行期保证同一时刻只有一个 borrow_mut，或任意个 borrow。
    let v = RefCell::new(vec![1]);
    v.borrow_mut().push(2); // 这个 Guard 在语句结束时 Drop，借结束
    assert_eq!(v.borrow()[1], 2);
    println!("RefCell 内容 = {:?}", v.borrow());

    // 同时借会 panic（不是编译失败）：
    // let _a = v.borrow();
    // let _b = v.borrow_mut(); // already borrowed
}
