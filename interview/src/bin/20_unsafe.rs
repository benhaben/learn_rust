//! 进阶：unsafe 边界
//!
//! 运行：`cargo run --bin 20_unsafe`
//!
//! `unsafe` **不关闭**借用检查，只是允许五件危险操作：
//! 解引用裸指针、调 unsafe 函数、读写可变静态、实现 unsafe trait、访问 union 字段。
//!
//! 纪律：
//! 1. unsafe 块尽量小。
//! 2. 把不变量写在 `/// Safety:` 里：调用方必须保证什么。
//! 3. 对外只暴露安全 API，让调用方无法打破不变量。
//!
//! 面试场景：FFI、自己写 Vec、无锁队列、SIMD。加分：Miri、文档、测试。
//! 「自己写 Vec」= Vec 内部就是堆缓冲 + 未初始化容量 + 裸指针，必须 unsafe；
//! 业务用现成 Vec 不必写。不操作内存就用不到。
//!
//! `safe_head` 和 `read_raw` 没有调用关系：前者是对照用的安全 API，
//! 后者才是裸指针。能让人走切片，就别把指针交给业务。
//!
//! 裸指针别名 + 同时可变 = UB，不是“看运气”。

/// # Safety
///
/// - `p` 必须指向一个已初始化、正确对齐的 `T`
/// - 在本次调用期间该内存可读，且没有别的 `&mut T`
unsafe fn read_raw<T: Copy>(p: *const T) -> T {
    // SAFETY: 由调用约定保证。
    unsafe { std::ptr::read(p) }
}

/// 对照：安全 API。不调用 read_raw，只演示「对外该长这样」。
fn safe_head(xs: &[u8]) -> Option<u8> {
    xs.first().copied()
}

fn main() {
    println!("安全 API: {:?}", safe_head(&[9, 8, 7]));

    let n = 42i32;
    // 从引用得到指针是安全的；解引用才需要 unsafe。
    let p = &n as *const i32;
    let v = unsafe { read_raw(p) };
    println!("unsafe 读到 {v}");
}
