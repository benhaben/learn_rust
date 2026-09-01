//! 进阶：孤儿规则 / newtype
//!
//! 运行：`cargo run --bin 24_orphan_newtype`
//!
//! **孤儿规则**：不能为外部类型 impl 外部 trait。
//! 否则全世界的 crate 都给 `Vec<T>` impl `Display`，连哪个 impl 都不知道。
//!
//! 绕法：在自己的 crate 里包一层 **newtype** `struct UserId(u64)`。
//! 零成本（布局和里面一样），但不会自动继承内部方法；需要的话再 `Deref`。
//!
//! 想给别人的类型加 `Serialize`：包 wrapper，或用远程 derive。

struct UserId(u64);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "u{}", self.0)
    }
}

// impl std::fmt::Display for Vec<u64> {} // 孤儿规则：Vec 和 Display 都不是你的

fn main() {
    let id = UserId(42);
    println!("newtype Display = {id}");
    println!("里面的值仍是 u64: {}", id.0);
}
