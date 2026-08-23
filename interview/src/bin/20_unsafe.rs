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

/// 安全封装：切片已经保证指针有效。
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
