//! 必考：Drop / RAII
//!
//! 运行：`cargo run --bin 10_drop`
//!
//! **RAII**：资源的寿命绑在值的寿命上。值离开作用域 → 调 `Drop::drop` → 关锁 / 关 fd / 释放堆。
//! 这就是“不用写 finally”的卖点。Mutex、File、BufWriter 都靠它。
//!
//! - `drop(g)`：提前释放，不必等花括号。
//! - `mem::forget(g)`：跳过 Drop，泄漏。有时故意用来“拆开”一个需要成对释放的结构。
//! - Drop 里再 panic：若已经在 unwind，会双 panic → abort。
//! - async 里把 Future Drop 掉 = 取消（不再 poll）。副作用不会自动回滚。

struct Guard<'a>(&'a str);

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        println!("Drop: 释放 {}", self.0);
    }
}

fn main() {
    {
        let _g = Guard("作用域结束");
        println!("还在用锁");
    } // 这里自动 drop

    let g = Guard("手动 drop");
    drop(g); // 提前释放；后面 g 不能再用
    println!("drop 之后已经没有锁了");

    let leaked = Guard("forget 不会打印释放");
    std::mem::forget(leaked);
    println!("forget 跳过了 Drop（演示泄漏，生产别乱用）");
}
