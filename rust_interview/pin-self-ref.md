# Pin、PhantomPinned、自引用 Future

对应：`cargo run --bin 28_pin`、`29_self_ref_fut`。运行时见 [tokio-future-pin.md](tokio-future-pin.md)。

## Pin / Unpin / PhantomPinned 不重复

| | 干什么 |
|---|---|
| `Pin<P>` | 钉子 API：通过这层指针不要把里面的值挪走 |
| `Unpin` | 「挪我也没事」。大多数类型默认有，此时 Pin 几乎没约束 |
| `PhantomPinned` | **写在类型定义里**的空字段，让结构体变成 `!Unpin`，钉子才生效 |

`PhantomPinned` 不钉地址、不占空间。钉某一个值用 `Box::pin` / `pin!`。

手写自引用或教学演示才加字段。`async` 生成的 Future 自己是 `!Unpin`。

口条：**Pin 钉值。PhantomPinned 改类型（我是 !Unpin）。**

## `28` 后半段不是 Pin 知识

`dummy_cx`：`poll` 要 `&mut Context`，里面要有 `Waker`。假造空的叫醒器，好编过。真项目 `tokio` 会给。

| | 作用 |
|---|---|
| `Waker` | `.wake()` 叫醒任务 |
| `RawWaker` + `RawWakerVTable` | 造 Waker 的指针 + 四个函数（clone/wake/drop） |

`Immobile` + `pin_box`：看 `!Unpin` 待在 `Pin<Box<_>>` 里。  
`_std_signature`：压警告，展示标准 `Future` 也是 `Pin<&mut Self>`。

线程 + 锁和 C/C++ 差不多。Pin/Waker 是 **async 状态机** 才有的名字。日常 `.await` / `spawn` 不必手写。C 里同类问题（回调用了栈指针）没有这些词。

## `29`：源码里没有 Pin 包 `s`，不是写错

要钉的是**整台状态机**（`s` 和指向它的指针在同一块），不是给 `s` 套 `Pin`。`hold().await` 时执行器已经 `Pin<&mut Fut>`。

```text
Fut
  s: String "tick"
  inner: echo 的 Future ──指针──→ 指回 s
  state
```

第一次 poll 后 `s` 进状态机，`echo(&s).await` 还握着 `&s`。再 move 整颗 Fut，指针悬空。

`async fn` → 编译器生成 `impl Future` 的状态机：跨 await 的**局部变量**成字段，函数体拆进 `poll` 的步骤。调用 `hold()` 只是造结构体；有人 poll 才跑。普通 `fn` 不会生成 Future。

口条：**async = 状态机 Future。自引用所以钉整颗 Future。业务源码里往往看不见 Pin。**
