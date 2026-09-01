# 宏、属性、thiserror、`#[from]`

对应示例：`cargo run --bin 08_result`（在 `interview/` 目录下跑）。

## `#[from]` 给谁看

只给 **`#[derive(Error)]`**（thiserror）看，`Debug` 不看。

```rust
#[derive(Debug, Error)]
enum LoadErr {
    #[error("读文件失败: {0}")]
    Io(#[from] std::io::Error),
}
```

- 字段运行时仍是 `Io(io::Error)`，`#[from]` 不是成员函数
- thiserror 的 derive 读到它，生成 `impl From<io::Error> for LoadErr`
- `?` 靠的是这个 `From`，不是属性自己会转
- 没有 `#[derive(Error)]`，单独写 `#[from]` 没有意义

`#[error("...")]` 同样是给 `Error` 看的：生成 `Display`。`{0}` 是第 0 个字段。

## thiserror 是什么

crates.io 上的库，提供 `Error` 这个 derive 宏。`Cargo.toml` 写上依赖，`use thiserror::Error` 后才能用。

- 库 crate：`enum` + thiserror + `From`，调用方能 `match`
- 应用：`anyhow::Error`，加 context
- `#[from]` 像 Java 注解；`#[derive(Error)]` 才是宏；thiserror 是提供宏的库

属性用完就扔，运行期没有反射里的 `#[from]`。

## 宏怎么分类（官方）

```text
宏
├── 声明宏  macro_rules!     println!  vec!
└── 过程宏  TokenStream → TokenStream（编译期函数）
      ├── 函数式      sqlx::query!
      ├── Derive      #[derive(Error)]
      └── 属性宏      #[tokio::main]
```

`#[...]` 是**属性语法**，和宏交叉，不是「宏包含 annotation」：

| `#[...]` | 例子 | 会不会当宏跑 |
|---|---|---|
| 编译器内置 | `#[test]` `#[cfg]` | 编译器自己处理 |
| 属性宏 | `#[tokio::main]` | 会 |
| Derive 的辅助属性 | `#[from]` `#[error]` | 不会单独跑，给 derive 读 |
| Derive | `#[derive(Error)]` | `Error` 是 derive 宏 |

口条：**宏分声明宏 / 过程宏；过程宏再分函数式、derive、属性宏。`#[ ]` 只是贴纸。**

## 和 C++ 怎么对上

不要想成 `#define`。Rust 没有预处理器，也不用头文件。

| C++ | Rust |
|---|---|
| `#define` / `#include` | 没有对等物 |
| `template` | 泛型（能用泛型就别上宏） |
| `[[nodiscard]]` | 内置属性 |
| Qt moc / codegen 插件 | 过程宏（依赖里的编译期插件） |

怎么读：

1. `名字!` → 会展开，不是函数
2. `#[derive(X)]` → 旁边生成 `impl`
3. 短的 `#[from]` → 给上面 derive 的元数据
4. `#[test]` / `#[cfg]` → 当 C++ 的 `[[ ]]` / `#ifdef`
5. 日常靠约定，不必 `cargo expand`

主线：运行时 → 函数；按类型生成 → 泛型；按语法生成 → 宏；给生成器/编译器的话 → `#[ ]`。

理解 `08_result` 先抓 `Result` + `From` + `?`，thiserror 只是少写 `impl`。

## `?` 为什么会调 `From`

这是**语言规则**，写在编译器里，不是 thiserror 的约定。对 `Result`，`?` 大致等于：

```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
}
```

编译器看：表达式是 `Result<T, F>`，函数返回 `Result<_, E>`，于是要求 `E: From<F>`。对得上就 `From::from` 再 `return Err`；对不上就编译失败。`F == E` 时走恒等 `From`，看起来像没转。底层是 `Try` trait，`Result` 的实现里仍然对错误做 `From`。

`#[from]` 只是生成那个 `impl`，好让约定对得上。

## `load_lib` 里两次 `?` 的流程

都不抛异常，只返回 `Err`。`?` 看见 `Err` 就转一层再 `return`。

文件不存在：

```text
read_to_string → Err(io::Error)
?              → From → LoadErr::Io → 直接 return，parse 不跑
```

内容不是数字（`08_result` 后半段）：

```text
read_to_string → Ok("not-a-number\n")
parse          → Err(ParseIntError)
?              → From → LoadErr::Parse → return
```

## 为什么要转成 `LoadErr`

函数签名只能有一种 `E`。底层却是 `io::Error` 和 `ParseIntError` 两种。转成自己的枚举，才能：

- 一个函数、一种错误、多种来源
- 调用方 `match Io / Parse`（重试 vs 内容坏了）
- `Display` 统一；`?` 自动转，不必每次 `map_err`

这就是 Rust 常规模式：**错误枚举 + `?` + `From`**。不转也可以手写 `map_err(LoadErr::Io)`。应用层不分种类才用 anyhow。

## 「invalid digit found in string」是谁打的

`println!("{e}")` 走 `LoadErr` 的 `Display`：`#[error("不是整数: {0}")]`。  
`{0}` 是里面的 `ParseIntError`，它的 `Display` 是标准库那句英文。

所以：「不是整数:」是你的模板；「invalid digit...」是 `parse::<i32>()` 失败的说明。要在 `interview/` 下 `cargo run --bin 08_result` 才能看到第三行；在仓库根目录 `cargo run` 会跑猜数游戏。
