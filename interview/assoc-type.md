# 关联类型 vs 泛型参数

对应：`cargo run --bin 16_assoc_type`。

## 两种都是「trait 里再出现一个类型」

实现 trait 时还要指定一个类型（节点是什么、从什么转过来）。  
差别是：**同一个实现者能不能选好几种。**

## 关联类型：一个实现者只许一种

```rust
trait Graph {
    type Node;                    // 由 impl 选定，不是调用方传入
    fn start(&self) -> Self::Node;
}
impl Graph for EmptyGraph {
    type Node = u32;              // 这份图的节点只能是 u32
}
```

不能再写一份 `type Node = String`。看见 `g.start()`，返回值已经定死，不用涡轮鱼。  
`Iterator::Item` 一样：`vec.iter()` 的元素不会又是 `i32` 又是 `String`。

## `()` 不是给括号重载

`()` 是 **unit（单元类型）**：没有内容的类型，只有一个值，也写成 `()`。接近空结构体。  
`impl Graph for ()` 是给这个空类型实现 Graph，图本身不是「括号结构」。更顺眼可以写成 `struct EmptyGraph;`。  
`().start()` 是对 unit 值调方法，不是运算符重载。

## 泛型参数：同一类型可以对很多 T 各 impl 一次

```rust
String::from("hi");  // From<&str>
String::from('x');   // From<char>
```

变的是**输入**，返回永远是 `String`。不是「泛型才能返回不同类型」。

若写成 `trait Graph<Node>`，可以同时 `impl Graph<u32>` 和 `impl Graph<String>`。  
那时 `().start()` 含糊，必须 `Graph::<u32>::start(&())`。

## 涡轮鱼

`::<>` 的绰号，用来手写类型参数。关联类型定死后往往不用写。

## 怎么选

| 关系 | 用谁 | 例子 |
|---|---|---|
| 这个类型当这个 trait 用时，那一项只有一种 | 关联类型 | `Iterator::Item`、`Graph::Node` |
| 同一类型要对多种「另一头」各成立一次（常常是输入） | 泛型参数 | `From<T>` |

口条：**一种输出用关联类型。同一 trait 要对多种输入成立，用泛型参数。**
