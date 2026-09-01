//! 进阶：FFI / repr(C)
//!
//! 运行：`cargo run --bin 21_ffi_repr_c`
//!
//! Rust 默认 `#[repr(Rust)]`：**字段顺序和对齐不稳定**，C 对不上。
//! 跨语言必须 `#[repr(C)]`，两边字段、类型、调用约定完全一致。
//!
//! - `String` / `Vec` 不能直接给 C：它们是 (ptr, len, cap)，不是 `char*`。
//! - 字符串走 `CString` / `CStr`（末尾 NUL，中间不能有 0）。
//! - 从 C 进来的指针立刻包进安全类型，不要满天飞 `*mut`。
//!
//! 本文件用“Rust 假装自己是 C 库”：`extern "C"` 定义 + 同文件 `#[no_mangle]` 实现，
//! 这样不用真的链一个 .so 也能编译。真对接交易所 SDK 时，一边是 .h/.so，一边是 `extern "C"`。
//!
//! 底层调 C 远多于 C++：FFI 只承诺 C ABI。C++ 要先包一层 `extern "C"`。

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tick {
    px: f64,
    qty: u32,
}

/// 模拟 C 库导出的函数。真项目里这个符号在 .so 里。
#[no_mangle]
pub extern "C" fn feed_tick(t: Tick) {
    println!("C ABI 收到 tick px={} qty={}", t.px, t.qty);
}

fn send(px: f64, qty: u32) {
    // 本文件里 feed_tick 是 Rust 写的 extern "C"，直接调是安全的。
    // 真项目写成 `extern "C" { fn feed_tick(t: Tick); }` 再调，才必须 unsafe。
    feed_tick(Tick { px, qty });
}

fn to_c_string(s: &str) -> CString {
    // 给 C 送字：拥有 + 末尾 NUL。中间有 \0 会失败。
    CString::new(s).expect("C 字符串不能包含内部 NUL")
}

fn from_c_ptr<'a>(p: *const c_char) -> &'a str {
    unsafe { CStr::from_ptr(p).to_str().expect("不是合法 UTF-8") }
}

fn main() {
    send(101.5, 3);

    let c = to_c_string("BTCUSDT");
    println!("CString 传给 C 的指针指向: {}", from_c_ptr(c.as_ptr()));
}
