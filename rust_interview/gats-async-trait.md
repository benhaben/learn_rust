# GAT 和 trait 里的 async fn

对应：`cargo run --bin 27_gats_async_trait`。关联类型基础见 [assoc-type.md](assoc-type.md)。

## `main` 是两个演示，不是一条链

```text
Windows + while next  → GAT（关联类型带寿命）
Dummy.get(7).await    → trait 里写 async fn
```

运行上没关系。都是「trait 上以前不好写」的考点，所以放一个文件。概念远亲：还出来的东西跟「这次借 self」有关。`Some`/`None` 是 `Option`：还有没有下一个。

## GAT 是什么

关联类型自己也能带 `'a` / 泛型。普通 `Iterator::Item` 是固定一种，不能写「跟这次 `&mut self` 同寿」。

`while let Some(p) = w.next()`：有下一块就解出 `p`；`None` 停。

## `'s` 和 `'a` 不是同一根

| | 绑谁 | 含义 |
|---|---|---|
| `'s` | `Windows { s: &'s str }` | 源字符串活多久（结构体自带） |
| `'a` / `'_` | 这一次 `next(&mut self)` | 还拿着返回值 = 还借着迭代器 |

`Self: 'a`：结构体（以及 `'s`）必须活过 `'a`。  
这个 `Windows` 例子用 `'s` 当 `Item` 也能编过；GAT 要覆盖**自己握着 buf、没有 `'s`** 的迭代器。

## 「借的是 self」

数据在结构体字段里（`buf`），`next` 还 `&self.buf[..]`。切片能用，是因为 `self` 还在、且这次 `&mut self` 还占着。没有外源 `'s` 可填。

返回值用参数的寿命，是规矩：

```rust
fn next(&mut self) -> Option<&[u8]>
// 等于
fn next<'a>(&'a mut self) -> Option<&'a [u8]>
```

从 `self` 掏出的引用，不能比这次传入的借用更久。

## 不是「next 完就不能 next」

`while` 里明明反复 `next`。卡住的是：**`p` 还活着时不能再 `next`。** 每圈结束 `p` 死，借还回去，再 `next` 合法。

```rust
let p1 = w.next();
let p2 = w.next();  // 编译失败：p1 还在
```

这是编译期借用，运行时没有 `'a` 锁。C++ 留下 `string_view` 再 `next`，编译器不管。

`Windows` 若返回 `&'s str`（只跟源字符串同寿）：`next` 一返回 `&mut self` 就结束，可以同时握两块窗口，「还借着迭代器」这句没了。自己的 `buf` 要 `&mut` 切片时必须跟 `&mut self` 走。

## `Item<'_>` 为什么没写 `'a`

`'a` 在 `type Item<'a>` 里声明。`next` 里 `'_` 接到 `&mut self` 的匿名寿命上：

```rust
fn next(&mut self) -> Option<Self::Item<'_>>
// 等于
fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>
```

`'_` 不是第三根寿命，是占位：「跟这次 `&mut self` 同一根」。

## async fn in trait

现在稳定。以前要 `fn get(&self) -> impl Future<Output = String> + '_` 或 `#[async_trait]` 装箱。和 GAT 同类：返回值跟借 `self` 有关。`dyn` 里若含 async，通常仍要装箱的 Future。

口条：**`'s` 是源数据。`'a`/`'_` 是这一次把迭代器借出去。还握着返回值就不能再 next；丢掉就能。和最后一行 async 无关。**
