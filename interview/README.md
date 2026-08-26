# Rust 面试复习示例

复习表里的代码都在这里，每个主题一个可执行文件，注释里讲清 move / Copy / 借用等概念。

```bash
cd interview

# 编译全部
cargo build --bins

# 只跑一个主题（文件名去掉 .rs）
cargo run --bin 01_ownership
cargo run --bin 06_arc_mutex
cargo run --bin 12_tokio_spawn
```

| 编号 | 级别 | 命令 |
|---|---|---|
| 01–14 | 必考 | `01_ownership` … `14_string_str` |
| 15–27 | 进阶 | `15_object_safety` … `27_gats_async_trait` |
| 28–34 | 压轴 | `28_pin` … `34_pyo3_gil` |

带 `tokio` 的 bin：`12_tokio_spawn`、`13_mutex_await`、`25_async_cancel`、`29_self_ref_fut`、`32_custom_future`。

笔记：

- [cell-rc-arc-mutex.md](cell-rc-arc-mutex.md)：Box / Rc / Arc / Cell / Mutex 怎么选
- [cpp-vs-rust-smart-pointers.md](cpp-vs-rust-smart-pointers.md)：和 C++ `shared_ptr` 的对照
- [unwrap-deref-mutex.md](unwrap-deref-mutex.md)：unwrap、`*`、MutexGuard、map/collect
- [macros-attributes.md](macros-attributes.md)：宏、属性、thiserror、`#[from]`
- [decl-macros.md](decl-macros.md)：macro_rules、过程宏 crate、不是正则
- [ownership-move-closures.md](ownership-move-closures.md)：主人 / 外号 / move / FnOnce·FnMut·Fn
- [drop-raii-ffi.md](drop-raii-ffi.md)：Drop、RAII、forget、FFI、ManuallyDrop
- [iterator-ownership.md](iterator-ownership.md)：into_iter、copied、turbo-fish
- [result-enum.md](result-enum.md)：Result 二选一、`?` 对齐 E
- [tokio-future-pin.md](tokio-future-pin.md)：tokio、Future、Pin、spawn
- [mutex-guard-await.md](mutex-guard-await.md)：Mutex vs Guard、不能跨 await
- [string-str-dst.md](string-str-dst.md)：String / &str、Deref、DST
- [object-safety.md](object-safety.md)：dyn Trait、对象安全、clone vs clone_box
- [assoc-type.md](assoc-type.md)：关联类型 vs 泛型、()、涡轮鱼
- [static-hrtb.md](static-hrtb.md)：'static、HRTB、for<'a>
- [cow.md](cow.md)：Cow、写时克隆、Borrowed vs Owned
- [typestate.md](typestate.md)：编译期状态机、PhantomData
- [unsafe.md](unsafe.md)：unsafe 边界、为何写 Vec 才需要
- [ffi-repr-c.md](ffi-repr-c.md)：repr(C)、CString 送字 / CStr 接字
- [deref.md](deref.md)：Deref 是包装类型，不是基本类型
- [async-cancel.md](async-cancel.md)：tokio select! ≠ 内核 select
- [gats-async-trait.md](gats-async-trait.md)：GAT、's vs 'a、trait 里 async
- [pin-self-ref.md](pin-self-ref.md)：Pin vs PhantomPinned、自引用 Future
- [custom-future.md](custom-future.md)：自定义 Future、wake 叫 tokio、Ordering
