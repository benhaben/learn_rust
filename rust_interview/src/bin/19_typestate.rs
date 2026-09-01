//! 进阶：类型状态——用类型记住「走到哪一步了」
//!
//! 运行：`cargo run --bin 19_typestate`
//!
//! # 要解决什么
//!
//! 订单必须先提交，才能成交。C++ / 运行期常见写法：
//! `if (state != Live) panic`。写错了要跑到那一行才炸。
//!
//! Rust 可以把「新建 / 已提交」做成**不同类型**：
//! `Order<New>` 和 `Order<Live>`。`fill` 只实现在 `Order<Live>` 上。
//! 对还没提交的订单调 `fill`，**编译就失败**，不用等运行。
//!
//! # 本文件怎么走
//!
//! ```text
//! Order::new(1)  →  Order<New>     只有 submit
//! o.submit()     →  Order<Live>    吃掉旧的 New，还你一个 Live
//! o.fill()       →  成交，Live 也消耗掉
//! ```
//!
//! `submit(self)` 的 `self` 是 move：旧的 `Order<New>` 用过就不能再用，
//! 不会出现「手里还握着未提交订单却去 fill」。
//!
//! `Order::new(2).fill()` 编不过：`new` 得到的是 `Order<New>`，上面没有 `fill`。
//!
//! # New / Live 为什么是空结构体
//!
//! `struct New;` `struct Live;` 没有字段，运行时不占空间（零大小类型）。
//! 它们不当数据用，只当**标签**，好让 `Order<New>` 和 `Order<Live>` 是两种类型。
//!
//! # PhantomData 是干什么的
//!
//! `Order` 的字段里只有 `id`，没有把 `S` 存进去。
//! 编译器会抱怨：类型参数 `S` 没用上。
//! `PhantomData<S>` 是零大小的标记字段，告诉编译器：
//! 「请按 `S` 区分这是哪种 Order」，即使内存里没有 S。
//! 运行时不加一个字节。
//!
//! # 什么时候别用
//!
//! 状态特别多、组合爆炸，就别硬上。
//! 跨 FFI / serde 时类型信息会丢，边界上还是要校验。
//!
//! PhantomData 源码是空结构体 + `#[lang = "phantom_data"]`，作用全在类型系统。
//! 想法不是 Rust 独有；move + 零大小标签让它好写、好拦。
//!
//! 口条：类型状态 = 编译期状态机。PhantomData = 给编译器看的空壳。

use std::marker::PhantomData;

/// 标签：还没提交。没有字段，不占空间。
struct New;
/// 标签：已经提交。
struct Live;

struct Order<S> {
    id: u64,
    _s: PhantomData<S>,
}

impl Order<New> {
    fn new(id: u64) -> Self {
        Self {
            id,
            _s: PhantomData,
        }
    }

    /// 吃掉 New，还一个 Live。旧变量不能再当未提交订单用。
    fn submit(self) -> Order<Live> {
        println!("订单 {} 提交", self.id);
        Order {
            id: self.id,
            _s: PhantomData,
        }
    }
}

impl Order<Live> {
    fn fill(self) {
        println!("订单 {} 成交", self.id);
    }
}

fn main() {
    let o = Order::new(1);
    let o = o.submit();
    o.fill();

    // Order::new(2).fill(); // 编译失败：New 状态没有 fill
}
