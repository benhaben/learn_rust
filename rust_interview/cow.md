# Cow（Clone-on-Write）

对应：`cargo run --bin 18_cow`。

## 名字

Cow 是牛，也是双关：**C**lone-**o**n-**W**rite（写时克隆）。

`use std::borrow::Cow` 只是把标准库类型引进来，方便写 `Cow`。不创建对象，也不启动拷贝。`std::borrow` 放「借用 / 拥有」相关类型。

## 不是 Rust 的缺陷

函数有时原样还 `&str`，有时必须改出新 `String`。一个返回类型只能写一种：

| 你返回 | 没改时 | 改了时 |
|---|---|---|
| 总是 `String` | 白拷一份 | 没问题 |
| 总是 `&str` | 没问题 | 新串在函数里，返回就悬空 |

C++ 也一样：一律 `std::string` 乱拷，或 `string_view` 指临时对象会悬空。  
`Cow` 用枚举把两条路装进同一种类型，不是语言坏了。

## 为了省分配，不是功能更多

C++ 直接新开 `string` 完全合法。Rust 里 `to_string()` 也行。  
`Cow` 只省「**没改那一次**」的堆分配和拷贝。每次都改、或不在乎这点分配，直接新 `String` 更简单。

热路径里无脑 `to_string()` 往往更常见。`Cow` 适合读多改少（规范化路径、配置、反序列化）。自己还多一个分支，不是无脑更快。

## 写时克隆怎么理解

先拿着 **Borrowed**（不分配）。**第一次要改**（或要拿走拥有值）时，才把字拷进自己的 `String`。

真正发生 `.clone()` 的是：

- `to_mut()`：还是借的 → 先 clone 成 `String` → 再改
- `into_owned()`：还是借的 → clone 一次得到 `String`

`Cow::Owned(s.replace(' ', "_"))` **没有** `.clone()`。`replace` 自己造了新 `String`，`Owned` 只是装箱。这个例子演示两种箱子，不是演示 clone 那一下。

```text
没改 → Borrowed(&str)     0 次分配
改了 → Owned(新 String)   和 C++ 新 string 一样
从借变成拥有 → clone      这才是名字里的「写时克隆」
```

口条：**能借就借，要改才拷。`replace` 是直接造新串；clone 发生在 `to_mut` / `into_owned` 遇上 Borrowed 时。**
