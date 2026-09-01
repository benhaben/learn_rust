# tokio、Future、Pin、spawn

对应：`cargo run --bin 12_tokio_spawn`。

## tokio 是什么，为什么是库

Rust **已有线程**：`std::thread::spawn`。tokio 不是代替线程，而是跑**大量等 I/O 的任务**（类似 Node 事件循环）。

定位接近 C：默认没有胖运行时。语言只提供 `async` / `.await` / `Future`；**谁 poll、socket 怎么等**放进库（tokio / async-std / embassy）。嵌入式、WASM、纯 CPU 不必链工作窃取线程池。

| 场景 | 用谁 |
|---|---|
| 网关、行情 | tokio 多线程 |
| 单片机 | embassy |
| 只要线程不算异步 | `std::thread`，不用 tokio |

口条：tokio = 库实现的运行时；`async` 只是 Future，不 poll 就不跑。

## Future

「还没做完的计算」：可反复 `poll` 的状态机，不是线程。

```rust
enum Poll<T> { Ready(T), Pending }

trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- `Pending` 时登记 `cx.waker()`，底层就绪后 `wake()`，执行器再 poll
- `async fn` 返回 `impl Future`，调用本身几乎不干活
- `.await` = poll 里面的 Future，Pending 则自己也 Pending

和线程比：协作式，等 I/O 时让出 worker，只占小状态机。

## 语言 / std / tokio 如何拼事件循环

```text
语言   Future、Pin、async 状态机
std    Future、Waker、Poll；阻塞 TcpStream；thread
tokio  执行器（谁 poll）+ reactor（epoll）+ 异步 TcpStream / sleep
```

std 的 `read` 卡住整个 OS 线程。tokio 的 socket 非阻塞，挂在 epoll 上：没数据 → Pending + Waker；epoll 可读 → wake → 再 poll → Ready。少数 worker 跑很多 Future，不是一连接一线程。

`#[tokio::main]`：建运行时，`block_on` 最外层 async main。

CPU / 同步文件用 `spawn_blocking` 或 `std::thread`，别堵 worker。

## Pin 是什么

`Pin<P>`：钉住之后地址不再移动（直到 Drop）。自引用 Future（后面的 await 指着前面的局部）被挪走会悬空，所以 `poll` 要 `Pin<&mut Self>`。

大多数类型 `Unpin`（挪了也没事）。生成的自引用 Future 常是 `!Unpin`。Pin 不是锁，不启动任务。

## Pin 什么时候出现在代码里

日常 `async` / `.await` / `tokio::spawn` **自己不用写 Pin**，执行器和编译器会钉。

你会看见 / 要写 Pin 的情况：

| 场景 | 代码里长什么样 |
|---|---|
| 看标准库签名 | `Future::poll(self: Pin<&mut Self>, ...)` |
| 手写 Future / Stream | `fn poll(self: Pin<&mut Self>, ...)` |
| 栈上自己 poll | `tokio::pin!(fut);` 或 `pin_mut!(fut)`，再 `fut.as_mut().poll(...)` |
| 堆上钉住 | `Box::pin(fut)` → `Pin<Box<F>>`，再交给自己的循环 |
| 库 API | 少数函数吃 `Pin<&mut Self>`（`StreamExt` 底层、自引用结构） |

`12_tokio_spawn.rs` 里**没有**手写 Pin：`spawn` 在运行时内部把 Future 钉住再 poll。面试能讲「为什么 poll 要 Pin」，写业务几乎只写 `.await`。

`Unpin` 的值可以用 `Pin::new(&mut x)`；`!Unpin` 必须 `pin!` / `Box::pin`，不能安全地再 move 出来。

## spawn / spawn_blocking

`tokio::spawn`：把 Future 丢进 I/O worker 当独立任务。要 `Send + 'static`（可能换线程、比当前函数活得久），常 `async move`。返回 `JoinHandle`，`.await` 取结果。一万个 spawn 仍是那几个 worker，不是一万个 OS 线程。

`thread::spawn` 才是真线程。

`spawn_blocking`：闭包丢进**阻塞线程池**（同步 CPU、同步文件、阻塞 C）。`.await` 等做完。别在 async 里死循环。

| | `tokio::spawn` | `spawn_blocking` | `thread::spawn` |
|---|---|---|---|
| 跑什么 | Future | 同步会卡住的活 | 同步 |
| 线程 | I/O worker | 阻塞池 | 新线程 |

口条：**Pin = 钉住好 poll。spawn = 进事件循环。spawn_blocking = 阻塞换道。**
