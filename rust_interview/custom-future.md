# 自定义 Future、executor、`Ordering`

对应：`cargo run --bin 32_custom_future`。原子序细节也可跑 `30_atomic_order`。

## 为什么要自定义 Future

业务几乎不用手写。`sleep` / `TcpStream` 已是 Future。  
本例拆协议：把 **OS 线程干完活** 接到 tokio 的 `.await`。面试讲 poll / Waker；库里 oneshot、定时器才自己 `impl Future`。

## executor 是谁、wake 叫醒谁

executor = 反复 `poll` 的运行时。本文件是 **`#[tokio::main]` 的 tokio worker**。

`w.wake()` **不是**唤醒 `thread::spawn` 那条线程（它立旗后就退出）。  
叫醒的是正在 `fut.await` 的任务：请 tokio 再 poll。

`Waker` 来自第一次 `poll` 时的 `cx.waker().clone()`。

## 执行顺序

```text
1. 造 flag / waker / ReadyFlag
2. thread::spawn：后台去 sleep 30ms
3. fut.await → 第一次 poll：旗 false，存 Waker，Pending
4. 线程：flag=true，wake()
5. 第二次 poll：旗 true → Ready
```

线程若先跑完，第一次 poll 就 Ready，wake 可有可无。所以 poll 里先看旗，存 Waker 后再看一次。poll 里不能阻塞。

口条：**spawn 立旗+叫一声；poll 看旗该不该结束。wake 叫 tokio，不是后台线程。**

## `load` / `store` 是谁的方法

`AtomicBool` 的方法（`std::sync::atomic`）。`flag` 是 `Arc<AtomicBool>`，`.` 经 Deref 点到里面。`Ordering` 是第二参数。

## `Ordering` 是什么（用价格举例）

内存序：这个原子读写之外，**周围其它内存**对另一线程何时可见。

```text
线程 A                    线程 B
PX = 101.5
旗 = true  ← Release      看见旗 true ← Acquire
                          再读 PX，必须是 101.5
```

旗 Relaxed、数据普通写：B 可能看见旗 true，PX 还是 0。x86 常碰巧过，ARM 易翻车。

- Relaxed：只管这个原子，计数器用
- Release 写 + Acquire 读：先写数据再立旗；对面见旗后再读数据
- `32` 里几乎只有旗，Acquire/Release 是配对习惯；只有旗时 Relaxed 也常够

口条：**先写数据，再 Release 立旗；Acquire 看到旗再读数据。`if flag.load(Acquire)` 仍是问旗是否 true。**
