# String、&str、Deref、DST

对应：`cargo run --bin 14_string_str`。

## 拥有 vs 借用

和 `Vec<T>` / `&[T]` 是同一套拆法。

| 拥有（可改、可长大） | 借用（不分配、跟 owner 走） |
|---|---|
| `String` | `&str` |
| `Vec<T>` | `&[T]` |

`&str` / `&[T]` 是**胖指针**：`(指针, 长度)`。不拥有数据。

| | `String` | `str`（通常写成 `&str`） |
|---|---|---|
| 谁的 | 你的，能 `push` | 借来的 |
| 栈上长什么样 | ptr + 长度 + **容量** | ptr + 长度（无容量） |
| 字面量 `"hi"` | 要 `.to_string()` 才上堆 | 直接是 `&'static str` |
| 子串 | 再 `clone` 才独立 | `&s[1..4]`，零拷贝 |

口条：**`String` 是带容量的 UTF-8 缓冲；`str` 是那段字本身。**

## `&String` 和 `&str` 不是同一种

`title(&owned)` 能编过，不是因为两者相等，而是 **Deref 强制转换**。

`String` 实现了 `Deref<Target = str>`：编译器把 `&String` 自动当成 `&str`。

```text
&String  →  Deref  →  &str
```

反过来不行：`fn f(s: &String)` 接不了 `"world"`。字面量不是 `String`，`str` 也不会 Deref 成 `String`。

API：参数能写 `&str` 就别写 `String`。调用方可以传字面量、`&String`、子串 `&s[1..]`。只有你要**存下来或返回新数据**才给 / 造 `String`。

## Deref 怎么实现的

`String` 就是「保证 UTF-8 的 `Vec<u8>`」：

```rust
pub struct String {
    vec: Vec<u8>,
}

impl Deref for String {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

// as_str：不拷贝。同一块堆字节，换标签叫 &str
unsafe { str::from_utf8_unchecked(self.vec.as_slice()) }
```

`deref` 不分配。`&String` 能传给 `fn title(s: &str)`，就是编译器调了这一下。

## 为什么要两种（不是闲的）

只留 `String`：看一眼、传参、切子串都得拥有或再分配；`"hello"` 也得先堆一份。  
只留 `&str`：字没地方住、不能 `push`、函数返回后栈没了就悬空。

C++ 也是两套：`std::string`（拥有）和 `string_view` / `const char*`（看一眼）。Rust 把拥有 vs 借用写进类型。

## DST

**DST = Dynamically Sized Type**：编译期不知道值占多少字节。

`i32`、`String` 大小固定（`String` 栈上永远三个 usize）。  
`str`、`[T]`、`dyn Trait` 内容可长可短，**不能** `let x: str`。必须用指针把「现在有多长」补上：

| DST | 指针 | 多带的信息 |
|---|---|---|
| `str` | `&str` | 指针 + 字节长度 |
| `[T]` | `&[T]` | 指针 + 元素个数 |
| `dyn Trait` | `&dyn Trait` | 指针 + 虚表 |

所以日常写的是 `&str`，不是 `str`。`String` / `Vec<T>` 自己带着长度，不是 DST。

C++ 数组退化成指针时丢掉长度；Rust 反过来：长度留在 `&str` / `&[T]` 里。

## UTF-8

`String` / `&str` 按**字节**索引。`s[0]` 或 `&zh[0..1]` 对多字节字符可能切在码点中间 → panic。用 `.chars()` / `.get()` / 保证落在字符边界的字节范围。

口条：**DST = 多长要到运行时才知道，必须用胖指针拎着。`&String` ≠ `&str`，但能当 `&str` 用。**
