# Iterator 三件套：是否「拿走 Vec」

对应：`cargo run --bin 11_iterator`。

## 「拿走」是什么意思

拿走的是**整只箱子**（`Vec` 的所有权），不是「看一眼里面的元素」。

`for x in v` 等价于 `v.into_iter()`：`v` 被 move，循环结束后原来的名字不能再用。每个元素变成循环变量 `x`（类型 `T`），由 `x` 当主人。

| 写法 | 方法 | `x` 的类型 | 循环后 `v` |
|---|---|---|---|
| `for x in v` | `into_iter()` | `T` | 没了 |
| `for x in &v` | `iter()` | `&T` | 还在 |
| `for x in &mut v` | `iter_mut()` | `&mut T` | 还在 |

还要继续用这个 `Vec`，写 `&v` / `&mut v`。确定用完再拆，才 `for x in v`。

## 不是缺陷

别的语言 `for (x : vec)` 默认拷贝或引用，箱子还在。Rust 默认不偷偷 `clone`，所以按值遍历 = 拆箱，避免 `String` 暗分配。

这经常是你要的：`for name in names { send(name); }` 没有 clone。

要留下箱子就借。要拿出元素、留下缓冲（容量还在）：

```rust
for x in v.drain(..) { }
v.push(1);  // v 还在，是空的，可继续用
```

C++ 也有 `make_move_iterator`。Rust 把「搬走整箱」写成最直的 `for x in v`。

口条：**不是缺陷；要留下箱子写 `&v`。** 全拿走 = 不再需要这个 `Vec`，元素找新主人。

## 惰性（同文件）

`map` / `filter` 不跑，`collect` / `sum` / `for` / `fold` 才拉元素。`copied()` 把 `&i32` 变成 `i32`（Copy）。
