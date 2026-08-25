# 声明宏 `macro_rules!`（不是正则）

对应：`cargo run --bin 22_macros`。属性 / thiserror 见 [macros-attributes.md](macros-attributes.md)。

## 两类宏

- **声明宏** `macro_rules!`：按图案抠 token，编译期展开。`vec!`、`println!`、本文件的 `myvec!`。
- **过程宏**：另一段程序吃 token。`#[derive(Debug)]`、`#[tokio::main]`。

宏不是函数：展开前没有类型。能用泛型 / 函数就别上宏。

## `$($x:expr),+ $(,)?` 怎么读

一条规则：左边图案，右边展开。

```text
(  $($x:expr),+    $(,)?  )  =>  {{ ... }}
     │      │  │      │
     │      │  │      └── 末尾可以多一个逗号
     │      │  └── 表达式之间用逗号隔开
     │      └── 每一段是表达式
     └── 重复至少一次（+）
```

| 写法 | 意思 |
|---|---|
| `$x:expr` | 抓一个表达式，名叫 `$x` |
| `$( ... ),+` | 重复一次或多次，中间 `,` |
| `$(,)?` | 可选的末尾逗号 |
| `$( v.push($x); )+` | 每抓到一个 `$x`，生成一句 `push` |
| `{{ ... }}` | 外层是宏括号；内层是代码块 |

`myvec![1, 2, 3]` 展开成 `push(1); push(2); push(3);`。  
`myvec![10, 20,]` 末尾逗号被 `$(,)?` 吃掉。空的走 `() => Vec::new()`。

## 不是正则表达式

只是 `+` / `?` 长得像。匹配的是 **Rust 语法块**，不是字符。

| | 正则 | `macro_rules!` |
|---|---|---|
| 吃什么 | 字符 | 表达式、类型、标识符 |
| `1 + 2` | 三个字符 | **一个** `$x:expr` |

口条：**`$()` 是重复；`:expr` 抓表达式。像正则的是 `+` `?`；对的是代码结构。**

## 过程宏 / 属性宏（和 `#define` 不是一类）

`macro_rules!` 像加强版 C 宏：看见图案，换成代码。  
**过程宏**是编译期跑的一段真正的 Rust 程序：吃 `TokenStream`，吐出新代码。

```text
过程宏
├── derive     #[derive(Debug)]      读结构体，生成 impl
├── 属性宏     #[tokio::main]        改掉整项（包一层 runtime）
└── 函数式     sqlx::query!("...")   长得像函数调用
```

第 7 行那三个都是过程宏，贴法不同。「属性宏」是过程宏的一种，不是并列的第三大类。  
`#[from]` 是 derive 宏读的属性，本身不是一种宏。

| | 接近 |
|---|---|
| `macro_rules!` | `#define`，但按语法抠，有卫生性 |
| 过程宏 | codegen / 编译器插件，不是 `#define` |

## 「单独开一个 proc-macro crate」

过程宏编出来是**给 rustc 用的插件**，必须先编好，业务才能用。同一个包既当普通库又当插件会成环，所以要另开一份 `Cargo.toml`：

```toml
[lib]
proc-macro = true
```

```text
my_app/      业务，依赖 my_macros
my_macros/   只有过程宏
```

`thiserror`、`tokio` 的 `#[derive(Error)]` / `#[tokio::main]` 住在它们的宏包里。  
手写 `myvec!`、用 `println!` 不必自己开包。自己写 derive / 属性宏才要。

口条：**`macro_rules` 是图案替换，写在当前文件即可。过程宏是编译期小程序，必须单独一个 crate。**
