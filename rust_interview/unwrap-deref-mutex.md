# unwrap、`*`、MutexGuard、map/collect

对应示例：`cargo run --bin 06_arc_mutex`。

## map 惰性、collect 才 spawn、join 只等待

```rust
let joins: Vec<_> = (0..n)
    .map(|_| {
        let hit = Arc::clone(&hit);
        thread::spawn(move || *hit.lock().unwrap() += 1)
    })
    .collect();
for j in joins { j.join().unwrap(); }
```

`map` **没有先造好一个函数数组**。它只是套一层迭代器，装着那一个闭包；没人要下一个元素时，闭包一次都不跑。

```text
(0..n).map(|_| spawn(...))   // 惰性：记下“每次要元素时就跑这闭包”
.collect()                   // 拉 n 次 → n 次 spawn，Vec 里是 JoinHandle
j.join()                     // 等那个子线程结束，不负责启动
```

- 会执行：`clone`、`spawn`、子线程里的 `+= 1`（可能 collect 还没完就已经在加）
- collect 里等不齐所有加完；那是后面的 `join`
- 若在 map 里立刻 `join`，会变成串行

口条：**`map` 记下怎么变；`collect` 真的变；`join` 等变完的人下班。**

## 这一行里谁是指针

```text
*  hit  .lock()  .unwrap()  += 1
│   │      │         │
│   │      │         └ unwrap：Result → MutexGuard（和 * 无关）
│   │      └ Arc 自动解引用成 Mutex，然后加锁
│   └ Arc<Mutex<i32>>：带原子计数的指针（像 shared_ptr）
└ 解的是 MutexGuard，露出 i32
```

- `Mutex::new(0)` 造的是「锁 + 里面的 i32」，**不是指针**
- `Arc::new` 把它放到堆上；`hit` 才是带计数的指针
- `lock()` 返回 `Result<MutexGuard<i32>, _>`：中毒（持锁线程 panic）时是 `Err`

## unwrap 和 `*` 是两层皮

| 你想干什么 | 用什么 | 接近 C++ |
|---|---|---|
| 把「可能失败」变成「失败就崩」 | `unwrap` / 或 `?` / `match` | `optional.value()`、不管错误码 |
| 调里面的方法 | 通常不用 `*`，`.` 会自动解引用 | `shared_ptr` 的 `->` |
| 改 / 读 / 搬走里面的值本身 | 要 `*` | `*p`、`(*p)++` |

`unwrap` 跟指针无关：拆的是 `Result` / `Option`。  
`+=`、赋值不会自动解引用，所以必须 `*guard`。  
`hit.lock()` 不用先 `*hit`：`.` 会一层层找方法（Arc → Mutex）。

## MutexGuard 不是指针，为什么还能 `*`

`MutexGuard<i32>` 是 RAII 锁：活着占锁，Drop 解锁。它不会算数，`+=` 是 `i32` 的。

它 **impl 了 `Deref` / `DerefMut`**，`Target = i32`。Rust 的 `*` 不是「认定这是指针」，而是「实现了 Deref 就能露出 Target」。

```rust
// 编译器把 *g += 1 大致做成：
*DerefMut::deref_mut(&mut g) += 1;
```

对应 C++：`unique_ptr` 重载 `operator*` / `operator->`。Rust 不随便重载 `*`，只走 `Deref` 这条通道。

| C++ | Rust |
|---|---|
| `unique_ptr` 重载 `*` / `->` | `Box` / `MutexGuard` impl `Deref` / `DerefMut` |
| `*p`、`p->foo()` | `*g`、`g.foo()`（`.` 自动 deref） |

C++ 里锁和数据常分开（`lock_guard` + 另一个 `int&`）。Rust 的 `lock()` 还给你 Guard，`*` 相当于取出那次 `int&`。

口条：**`unwrap` 拆错误包装；`*` 拆 Deref 包装；`.` 常常替你拆一层。** Guard 是锁的壳，壳不会 `+`。
