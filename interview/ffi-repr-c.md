# FFI、`repr(C)`、调 C 还是 C++

对应：`cargo run --bin 21_ffi_repr_c`。

## 和 C 哪些地方不一样

不写 `#[repr(C)]` / `extern "C"` 时，Rust **不是** C 的布局和调用方式。

| | C | Rust 默认 |
|---|---|---|
| 结构体 | 按源码顺序，对齐固定 | `repr(Rust)`，字段可重排，C 对不上 |
| 字符串 | `char*`，`'\0'` 结尾 | `String` / `&str` 是 ptr+长度（`String` 还有容量） |
| 动态数组 | 指针 + 自己记长度 | `Vec` 是 ptr+len+cap，不能当 `T*` 扔过去 |
| 函数名 | 基本不改编 | 会改编；给 C 用要 `#[no_mangle]` + `extern "C"` |
| 调用约定 | C ABI | Rust 自己的 ABI |

要对齐：`#[repr(C)]` 结构体、`extern "C"` 函数、`CString` / `CStr`（补 NUL，中间不能有 0）。  
`f64` / `u32` 和 C 的 `double` / `uint32_t` 对应，这倒一样。

从 C 来的指针立刻包进安全类型，不要满天飞 `*mut`。

本文件是 Rust 假装自己是 C 库（同文件 `#[no_mangle] extern "C"`），直接调不必 `unsafe`。真 SDK：一边 `.h` / `.so`，一边 `extern "C" { fn ... }`，调用通常要 `unsafe`。

## 底层调 C 多还是 C++ 多

**C 多得多。** OS、`libc`、交易所 C SDK 都是 C ABI。Rust 标准库底层也调这些。

C++ 几乎不直接调：名字改编、`this`、异常、模板，每个编译器一份。Rust FFI 只承诺 **C**。有 C++ 库，先包 `extern "C"`，或用 `cxx`；量化里常见 C++ 引擎导出 C 接口，Rust 网关来调。

口条：**跨语言对齐的是 C，不是 C++。默认布局和 `String` 都不是 C；`repr(C)` + `extern "C"` + `CString` 才对得上。**

## `CString` / `CStr`

`CString` 是给 C 函数用的、**自己拥有**的字：堆上拷一份，末尾补 `'\0'`，`as_ptr()` 才能当 `char*`。

| | Rust `&str` / `String` | `CString` |
|---|---|---|
| 结尾 | 靠长度，没有必须的 NUL | 保证以 `'\0'` 结尾 |
| 中间能不能有 `0` | 能 | 不能（C 会当成结束；`CString::new` 会失败） |
| 传给 C | 对不上 | `c.as_ptr()` → `*const c_char` |

`CStr` 是**借用**的 C 字符串：C 已经有一块 `char*`，用 `CStr::from_ptr` 看过来，不拥有。

口条：**给 C 送字用 `CString`；从 C 接字用 `CStr`。**
