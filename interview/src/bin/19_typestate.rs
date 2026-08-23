//! 进阶：类型状态（typestate）
//!
//! 运行：`cargo run --bin 19_typestate`
//!
//! 用**类型参数**表示对象处于哪一阶段。非法转移在编译期失败，
//! 而不是运行期 `if state != Live { panic }`。
//!
//! 手法：
//! - `New` / `Live` 是零大小标记类型（没有字段，不占空间）。
//! - 方法吃 `self`（move），返回新状态的 `Order<Next>`。旧值不能再用。
//! - `PhantomData<S>` 告诉编译器“我们按 S 区分类型”，即使 S 没出现在字段里。
//!
//! 跨 FFI / serde 时类型状态会丢，边界上还是要校验。状态太多就别硬上。

use std::marker::PhantomData;

struct New;
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
