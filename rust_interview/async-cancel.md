# async 取消和 `tokio::select!`

对应：`cargo run --bin 25_async_cancel`。运行时见 [tokio-future-pin.md](tokio-future-pin.md)。

## 和底层 `select` / `epoll` 不一样

POSIX `select` / `epoll`：同时盯一批 fd，谁就绪就处理谁，**别的 fd 还在集合里**，不会因为你读了 A 就把 B 的等待拆掉。

`tokio::select!`：同时等几路 **Future**，谁先完成用谁，**没赢的 Future 被 Drop**。  
Drop = 不再 `poll` = 这次异步操作取消（定时器从 tokio 摘掉、这次 `read` 不再挂在你任务上）。

| | 内核 `select`/`epoll` | `tokio::select!` |
|---|---|---|
| 等的是 | fd / 事件 | Future（状态机） |
| 一路就绪后 | 其它路通常还在盯 | 其它路的 Future **析构** |
| 「取消」 | 你自己 `FD_CLR` / 不读 | 默认就取消没赢的那几路 |

口条：**内核 select 是「谁好处理谁，其余接着等」。`tokio::select!` 多一步：没选中的 Future 扔掉。**

## 取消的是哪一层

| 层 | 会不会停 |
|---|---|
| 这个任务还等不等（sleep、这次 read） | **会**。没赢的 Future 丢了 |
| 已经发出去的包 / 交易所是否收到 | **不会收回**。不是事务 |
| socket 还开不开 | 看你还握不握着 `TcpStream`。丢掉的是这一次 Future，不是自动 `close` |

超时包一层 ≠ 回滚。要自己做 client order id + 查询 / 幂等。

`25`：`sleep(50ms)` 先好，`do_io` 里 5 秒 sleep 被 Drop，「do_io 完成」不会打印。

口条：**先好的留下；其它路不再等。副作用要自己善后。**
