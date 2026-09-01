# 类型状态（typestate）和 PhantomData

对应：`cargo run --bin 19_typestate`。

## 是什么

把状态机的转移做成**类型转换**。非法转移在编译期当「没有这个方法」拦下来，而不是运行期 `if state != Live { panic }`。

```text
Order<New>  --submit(self)-->  Order<Live>  --fill(self)-->  结束
```

`fill` 只写在 `impl Order<Live>`。`Order::new(2).fill()` 编不过。

| 常见状态机 | 类型状态 |
|---|---|
| 一个对象 + `enum` 当前状态 | 每个状态一个类型 |
| 方法里 `if` 检查 | 方法只存在于合法状态上 |
| 转移后还是同一个变量 | `self` move，旧类型作废 |

运行期才知道的事（交易所拒单）仍用 `Result`。状态特别多、跨 FFI / serde 时类型会丢，别硬上。

口条：**类型状态 = 编译期状态机。合法转移换类型；非法转移是类型错误。**

## 是不是 Rust 特有

想法不是独有（C++ 两个类 / 模板、Haskell 幽灵类型，typestate 一词 80 年代就有）。

Rust 特别顺：默认 **move**（旧阶段作废）、**零大小标签**（不加运行时字段）、没有这个方法就是没有。C++ 容易拷一份未提交订单留下；要禁拷贝才接近。

## PhantomData 干什么

`Order` 内存里只有 `id`，`S` 不当数据存。没用上的类型参数编译器会拒。  
`PhantomData<S>` 是零大小假字段：告诉编译器「按 `S` 区分类型 / 算拥有 `S`」，运行时 0 字节。

口条：**给编译器看的空壳。让 `S` 区分类型，但不占内存。**

## PhantomData 怎么实现的

标准库就是空结构体，没有字段、没有运行时代码：

```rust
#[lang = "phantom_data"]
pub struct PhantomData<T>;
```

`size_of == 0`，`align_of == 1`。`Eq` 永远相等。构造写 `PhantomData` 即可。

`#[lang = "phantom_data"]` 让编译器特殊对待：没用的类型参数算用过、Drop 检查、协变。不生成机器码。`Order<New>` 和 `Order<Live>` 运行时都只是一个 `u64`。

口条：**空结构体 + 编译器认这个 lang item。作用全在类型系统。**
