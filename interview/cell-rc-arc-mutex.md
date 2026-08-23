# Box / Rc / Arc / Cell / Mutex 怎么选

C++ 里万事 `shared_ptr` 往往能编过。Rust 把那一包拆开，好让编译器检查线程和可变。不是为了多造名词。

## 一张表

| 你实际在做的事 | Rust | C++ 里常被 `shared_ptr` 混过去 |
|---|---|---|
| 只我一个人要 | `Box<T>` | 其实该用 `unique_ptr` |
| 同线程、好几个人要（只读） | `Rc<T>` | 仍用 `shared_ptr`（原子计数，偏贵） |
| 跨线程、好几个人要（只读） | `Arc<T>` | `shared_ptr` |
| 同线程、拿不到 `&mut` 还要改 | `Cell` / `RefCell` | 直接 `*p = ...` |
| 跨线程还要改 | `Arc<Mutex<T>>` 或 `Atomic*` | `shared_ptr` + 自己记得加锁 |

`i32` 和结构体规则一样：跨线程改不是「原始类型用 Mutex、结构体用 Cell」。跨线程改一律 `Mutex` / 原子；`Cell` 只用于单线程。

## 三个类型各管什么

| | 干什么 | 数据在哪 | 几个 owner | 有 `&` 时能不能改 |
|---|---|---|---|---|
| `Box<T>` | 唯一所有权 | 堆 | 1 | 要 `&mut` |
| `Rc<T>` | 共享所有权，数有几个人握着 | 堆 + 非原子计数 | 多个 | 默认不能 |
| `Arc<T>` | 同上，计数原子，能过线程 | 堆 + 原子计数 | 多个 | 默认不能 |
| `Cell<T>` | 内部可变：`&Cell` 就能换里面的值 | 不一定上堆 | 还是外面那个 owner | 能，`get` / `set` |

`Box` 管存放，`Rc`/`Arc` 管共享所有权，`Cell` 管「共享借也能改」。可以叠：`Rc<Cell<i32>>`、`Arc<Mutex<T>>`。

## Cell 是什么、不是什么

为了在只有 `&self` 时还能改数据。`get` / `set` 的签名是 `&self`，里面用 `UnsafeCell` 对裸指针读写，**不是**偷偷短借一下 `&mut` 再结束。

所以：

- 两个 `&Cell` 可以同时存在，各自 `set` 也能编译过
- 它不是 `Sync`：两个线程同时 `set` 就是数据竞争
- `get` 是 Copy 出一份，`set` 是整颗换掉，适合 `i32` / `bool`
- `String` / `Vec` / 图节点用 `RefCell`（运行期借检查，冲突 panic）

`inc` 可以是 `&self`，调用处的绑定也不需要 `let mut`。可变藏在 `Cell` 里。

能写成 `&mut self` 就写 `&mut self`。Cell 是绕开「`&` 不能改」，不是默认写法。

## 什么时候必须用 Cell / RefCell

判据：**这时候你拿不到 `&mut`，但还是要改一个字段。**

用不到（直接 `let mut` / `&mut self`）：

```rust
let mut n = 0;
n += 1;

struct S { n: i32 }
impl S {
    fn inc(&mut self) { self.n += 1; }
}
```

必须用：值已经在 `Rc` 里，而且不止一个句柄。

```rust
struct Counter { n: Cell<i32> }  // 若 n: i32，下面加不了
impl Counter {
    fn inc(&self) { self.n.set(self.n.get() + 1); }
}

let a = Rc::new(Counter { n: Cell::new(0) });
let b = Rc::clone(&a);  // 两个 owner，Rc::get_mut 失败
a.inc();                // 只有 Rc / &Counter，没有 &mut Counter
```

必须用：遍历时已经是 `&self`，还要改自己。

```rust
struct Node {
    hits: Cell<u32>,
    next: Option<Rc<Node>>,
}
fn walk(n: &Node) {
    n.hits.set(n.hits.get() + 1);
    if let Some(nx) = &n.next { walk(nx); }
}
```

`next` 是共享 `Rc`，做不到「整条链的 `&mut`」。`Rc::clone` 是多个句柄指向同一块，只给你 `&T`。只要计数 > 1，就没有唯一 owner。

图、observer 同一类：A 指着 B，B 指着 A，谁都没有独一无二的 `&mut`。

## 单线程为什么还要 Rc 引用计数

单线程 ≠ 只有一个变量握着这份数据。同一线程里可以有很多个 `Rc` 指向同一块（树里两个父节点共一个子、HashMap 一份 + 当前选中再握一份）。没有计数，谁 `drop` 谁释放，另一个还指着就悬空。

`Rc` 比 `Arc` 便宜，只是 `++`/`--` 不用原子指令，不是「单线程所以不用数」。

## 和 C++ shared_ptr

`shared_ptr` 拷贝就能共享、就能改 `*p`。计数安全，里面的 `T` 照样数据竞争。

Rust 不让这件事默默发生：共享一层，改另一层，跨线程再加 `Send`/`Sync`。概念多，是把 C++ 的「别写错」提前到类型上。

更短的 C++ 对照见 `cpp-vs-rust-smart-pointers.md`。

## 面试口条

- `Box` 管存放，`Rc` 管单线程多主人，`Arc` 管跨线程多主人
- `Cell` 绕的是「`&` 不能改」，不是偷偷 `&mut`；跨线程改用 `Mutex`/`Atomic`，别用 `Cell`
- 先问能不能名正言顺拿到 `&mut`：能就别上 Cell
- `shared_ptr` 把所有权和线程安全搅在一起；Rust 拆开，所以类型多，事故少
