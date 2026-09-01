# Mutex、Guard、不能跨 .await

对应：`cargo run --bin 06_arc_mutex`、`cargo run --bin 13_mutex_await`。

## Mutex 和 Guard 不是同一个东西

| | 是什么 | 类比 |
|---|---|---|
| `Mutex<T>` | 那把锁 + 被保护的数据 | 账本柜子 |
| `MutexGuard` | 「我正在开着柜子」的凭证 | 插在锁孔里的钥匙 |

`lock()` 成功 = 拿到 Guard = **已经上锁**。Guard 活着，锁就占着。Guard **Drop** = 自动 `unlock`。

口条：**Mutex 是锁，Guard 是持锁凭证。没 Drop 就是没释放。**

## 多线程共用一把锁：天天有

几个线程改**同一份**仓位 / 订单簿 / 计数，必须一份数据，否则各改各的对不上。

- **`Arc`**：每人一把指向**同一把锁**的指针
- **`Mutex`**：同一时刻只有一个人能打开

`06`：8 个线程对同一个 `Arc<Mutex<i32>>` 各 `+1`，结果一定是 8。

`13` **不禁止**这件事。禁止的是：**握着 Guard 去 `.await`**。

## `right` 和 `_wrong`

`right`：先拷数据，Guard 在花括号结束时 Drop（解锁），**再** `sleep().await`。

```rust
let n = { *m.lock().unwrap() }; // Guard 已死，n 只是 i32
tokio::time::sleep(...).await;
```

`_wrong`：`let _g = lock()` 后立刻 await。`_g` 活到函数结束，睡觉期间锁一直占着。

| | `right` | `_wrong` |
|---|---|---|
| `.await` 时 | 已经解锁 | 还握着 Guard |
| 别人 | 不用等你睡醒 | 堵在门口，等你被 wake 且 Guard Drop |
| 编译 | 过 | `MutexGuard` 不是 `Send`，打开 `sleep` 就挂 |

别人只能等，是因为**锁没还**，不是因为「await 本身禁止共用 Mutex」。

多线程 tokio 醒来可能换 worker。`std::sync::MutexGuard` 规定谁 lock 谁 unlock、不能换线程（`!Send`），所以常**直接编不过**。就算能编过，拿着锁睡觉也是事故：行情 / 风控全冻住。

## 两把锁怎么选

| 场景 | 用谁 |
|---|---|
| 多线程短改（加仓、计数），临界区里不 await | `Arc<std::sync::Mutex<T>>`（默认） |
| **必须**握着锁去读网络（少见） | `tokio::sync::Mutex`（Guard 可跨 await，但等 I/O 期间别人照样进不来） |

能拷出来再 await，就别用异步锁。量化事故：持锁做 CPU、serde、或回调 Python。

口条：**多线程共用一把锁 = `Arc` + `Mutex`。`13` 只禁「握着钥匙去午睡」。**
