# Rust 智能指针并不比 C++ 更好写

Rust 卖的不是「比 `unique_ptr` / `shared_ptr` 更省事」，而是把 C++ 里靠纪律的事变成编译期错误。

## 对应关系

| C++ | Rust |
|---|---|
| `unique_ptr<T>` | `Box<T>` |
| `shared_ptr<T>`（原子计数） | `Arc<T>` |
| 没有标准的非原子 `shared_ptr` | `Rc<T>`（单线程更便宜） |
| 要改共享数据：自己加 mutex | 还要再包 `Mutex`，而且 `Send` / `Sync` 会逼你选对 |

## 手感

C++ 往往写着更顺：`shared_ptr` 拷贝就能共享，函数随便传，线程里丢一份也编译过。

Rust 要 `Arc::clone`、跨线程不能用 `Rc`、共享可变还得 `Mutex`，第一周会觉得啰嗦。

## C++ 省掉的键，代价是静默事故

- 两个线程拿着 `shared_ptr` 同时改 `*p`：计数安全，**数据竞争照样 UB**
- `unique_ptr` move 之后还能误用空的（运行期）；Rust move 后直接不能用
- 循环引用两边都会泄漏；Rust 至少逼你看见 `clone`，C++ 拷贝太顺，更容易绕成环

## 面试口条

不是「比 C++ 简单」，是**把坑从运行期挪到编译期**。热路径、多线程、交易这种你愿意多写几行换不掉数据的，这个交换值。写个单线程工具，C++ `shared_ptr` 手感确实更轻。

智能指针模型差不多；Rust 多出来的 `Rc` / `Arc` 分家和 `Send` / `Sync`，换的是「共享可变必须过锁」，不是 API 更好看。

何时必须用 `Cell` / `Rc` / `Mutex`，见 `cell-rc-arc-mutex.md`。
