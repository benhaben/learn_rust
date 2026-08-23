//! 进阶：对象安全
//!
//! 运行：`cargo run --bin 15_object_safety`
//!
//! `dyn Trait` 是胖指针：`(数据指针, vtable)`。vtable 里每条方法必须能
//! **在不知道具体类型大小 / 具体类型名的情况下调用**。这叫对象安全。
//!
//! 常见违规：
//! - 方法返回 `Self`（不知道返回多大，也无法放到同一个 trait object 里）
//! - 方法带额外泛型参数（每种 T 都要一条 vtable 项，无穷多）
//!
//! 绕法：把这些方法标 `where Self: Sized`，它们不进 vtable，`dyn Trait` 仍可用其余方法。
//! 所以 `Clone` 默认不能 `Box<dyn Clone>`（`clone(&self) -> Self`）。

trait Work {
    fn run(&self);

    /// 想“克隆一个 trait object”：返回盒子，不要返回 Self。
    fn clone_box(&self) -> Box<dyn Work>;

    /// 泛型方法：加上 Sized，从 vtable 摘掉。只有在具体类型上能调。
    fn helper<T: std::fmt::Debug>(&self, v: T)
    where
        Self: Sized,
    {
        println!("helper 只能在具体类型上调, 收到 {v:?}");
    }
}

struct Ping;

impl Work for Ping {
    fn run(&self) {
        println!("Ping::run");
    }
    fn clone_box(&self) -> Box<dyn Work> {
        Box::new(Ping)
    }
}

fn main() {
    let w: Box<dyn Work> = Box::new(Ping);
    w.run();
    let w2 = w.clone_box();
    w2.run();

    // w.helper(1); // 不行：dyn Work 的 vtable 里没有 helper
    Ping.helper(1); // 具体类型可以

    // let _c: Box<dyn Clone> = Box::new(1); // clone 返回 Self，不是对象安全
}
