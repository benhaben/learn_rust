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

`map` / `filter` 不跑，`collect` / `sum` / `for` / `fold` 才拉元素。

## `copied()`：`&i32` 和 `i32` 是不同类型

`i32` 是四个字节的值；`&i32` 是指向它的只读外号。`Vec<i32>` ≠ `Vec<&i32>`。

`xs.iter()` 的 Item 是 `&i32`。要 `collect` 成 `Vec<i32>`，必须在某处变成 `i32`：`.copied()`、`.map(|&x| x)` 或 `.cloned()`。不写的话 `collect` 得到 `Vec<&i32>`，和返回类型对不上，编译器会报错——**看见类型对不上再加**，不必提前背。

`filter` 可以写在 `copied` 前或后；`filter` 的闭包拿到的是 `&Item`。

### 为什么叫 `copied` 而不是「copy &类型」

`&i32` **自己已经是 Copy**：再 Copy 一次只是多一根指针，类型仍是 `&i32`。

`copied()` 复制的是 **`&` 后面的 `T`**（`T: Copy`），Item 从 `&T` 变成 `T`。

| 方法 | 要求 | 结果 |
|---|---|---|
| `copied()` | `T: Copy` | `&T` → `T`，按位拷所指的值 |
| `cloned()` | `T: Clone` | `&T` → `T`，`String` 会分配 |

口条：**`Copy &i32` = 再拷一根指针；`copied()` = 拷那个 `i32`。** `i32` 用 `copied`；`String` 要拥有值用 `cloned` 或 `into_iter`。

## turbo-fish：`::<>`

社区绰号，不是关键字。表达式里给方法补泛型要用 `::<>`，否则 `parse<i32>()` 会和小于号打架。类型位置仍写 `Vec<i32>`。

```rust
s.parse::<i32>()
.collect::<Vec<_>>()   // Vec 定死，元素 _ 请推断
```

`parse` 返回 `Result<i32, ParseIntError>`，所以例子里 `.unwrap()`。`unwrap` 是 `Result`/`Option` 的**方法**，不是 trait。正经写法见 `result-enum.md`。

`evens` 的 `collect()` 能从返回类型 `Vec<i32>` 推断，不用写。`let parsed = ...collect()` 两边都没写容器，就要 turbofish 或 `let parsed: Vec<i32> =`。
