# 对象安全、`dyn Trait`、`Self: Sized`

对应：`cargo run --bin 15_object_safety`。

## 想解决什么

C++：基类指针指向子类，运行时查虚表。  
Rust 没有继承：几个类型都 impl 同一个 trait，装进同一种指针 `Box<dyn Work>`，调用时查 vtable。

## 为什么是 `Box<dyn Work>`，不是 `Box<Work>`

`Box<T>` 的 `T` 必须是**类型**（内存长什么样）。  
`Work` 是 **trait（契约）**，不是一种数据布局。没有「一个 Work 值」，也就没有 `Box<Work>`。

`dyn Work` 才是类型：**某个实现了 Work 的值，具体是谁已擦掉**。不同 impl 体积可以不同，所以它是 DST，必须用指针：`Box<dyn Work>` / `&dyn Work`。

| | 是什么 | 指针 |
|---|---|---|
| `Work` | 契约，不是类型 | 不能 `Box<Work>` |
| `Ping` | 具体类型，大小已知 | `Box<Ping>`（瘦） |
| `dyn Work` | 擦掉具体类型后的 DST | `Box<dyn Work>`（胖：数据 + vtable） |

`dyn` 写在类型里，避免看成「有个叫 Work 的结构体」。老 edition 曾允许省略。

`Box<dyn Clone>` 看起来怪，而且**本来就不合法**：`Clone::clone` 返回 `Self`，不能当 dyn 用。那是反例，不是推荐写法。

口条：**`<>` 里是类型。trait 名要先变成 `dyn Trait`，才是类型。**

## 对象安全

当 `dyn Trait` 用时，编译器要造一张**有限、对所有 impl 统一**的 vtable。  
进表的方法必须能在「只知道是这个 trait，不知道具体类型、不知道多大」时调用。做不到 → 整个 trait 不能 dyn。这叫不是对象安全。

常见违规：

1. **返回 `Self`**（或按值收 `Self`）。`clone(&self) -> Self` 的返回值该留多大，取决于具体类型。手里只有 `dyn Trait`，接不住。所以没有 `Box<dyn Clone>`。绕法：返回 `Box<dyn Trait>`（本文件的 `clone_box`）。盒子大小固定。
2. **方法自己再泛型** `fn helper<T>(...)`。每种 `T` 一条函数，vtable 无穷。编译 dyn 时不知道你会传哪种 `T`。

口条：**`dyn` 靠一张有限的 vtable。返回 `Self`、方法再泛型，表造不出来。**

## `Self: Sized` 是什么、为什么要有

`Sized`：编译期知道这个类型占多少字节。`Ping` 有；`dyn Work` 没有（DST）。

写在方法上：

```rust
fn helper<T>(...)
where
    Self: Sized,
```

意思是：**只有 `Self` 大小已知时，这个方法才存在。**

| 调用 | `Self` | `helper` |
|---|---|---|
| `Ping.helper(1)` | `Ping`（`Sized`） | 在，静态调用 |
| `w.helper(1)`，`w` 是 `dyn Work` | 不是 `Sized` | 当这个方法不存在 |

它不是运行时检查，是编译期开关：这条方法要不要参加「当 dyn 用」那一套。

**没有这句：** `helper<T>` 让整个 `Work` 不是对象安全，连 `Box<dyn Work>` 都不能写，`run` 也搭进去。  
**有了这句：** `helper` 不进 vtable；`dyn Work` 上没有它；其余方法仍对象安全。

不想写这句，就把泛型方法拆到另一个 trait。效果类似：`dyn Work` 上看不到它。

口条：**`Self: Sized` =「这方法只给具体类型，不要放进 dyn」。没有这句，泛型方法会毁掉整个 `dyn Trait`。**
