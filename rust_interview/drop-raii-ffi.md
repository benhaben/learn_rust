# Drop、RAII、forget、FFI

对应：`10_drop`、`21_ffi_repr_c`。

## RAII：C++ 也有

资源绑在对象寿命上，析构时释放。Rust 的 `Drop` 对标 C++ 析构函数。`lock_guard` ≈ `MutexGuard`。

差别：C++ move 之后源对象往往还在，析构仍会跑（要能处理空）；Rust move 后旧绑定作废，只由新主人 `Drop`。Drop 里再 panic，unwind 中会 abort。

## `drop` vs `forget`

| | 所有权 | `Drop` |
|---|---|---|
| 离开作用域 | 没了 | 会跑 |
| `drop(g)` | 没了 | **会跑**（提前释放） |
| `mem::forget(g)` | 没了 | **不跑**（泄漏） |

`forget` 把值收走且不析构。`forget(String)` 会泄漏堆。日常要提前释放用 `drop`，不要 `forget`。

`forget` 的正当理由：析构权交给外面，避免两边都 `free`。例如把 `Vec` 拆成指针给 C。

## FFI

Foreign Function Interface：按约定调别的语言，Rust 里通常是 C ABI（`extern "C"`、`#[repr(C)]`）。`String`/`Vec` 对 C 的 `char*`/`malloc`，所有权和布局必须对齐。交易所 C SDK、pyo3 都算 FFI。

「拆 Vec 交给 C 再由 C `free`」几乎只出现在 FFI 或自己写 unsafe 容器。普通单语言业务不用 `forget`。

## `ManuallyDrop<T>`

包装一层，里面的 `T` **默认不自动 Drop**。你还能继续用这个值（`Deref`），并自己选时候：

- `ManuallyDrop::into_inner(v)` 拿回 `T`，之后正常 Drop
- `unsafe { ManuallyDrop::drop(&mut v) }` 就地析构一次

比 `forget` 合适的地方：壳还留着、字段要挪走、不能让旧壳再析构一次。

口条：**RAII 两边都有。`drop` = 现在析构；`forget` = 不析构。`ManuallyDrop` = 先别自动析构。FFI = 跨语言，这才是 forget 的主场。**
