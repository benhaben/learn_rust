//! 进阶：泛型单态化 vs dyn
//!
//! 运行：`cargo run --bin 26_dyn_vs_generic`
//!
//! - **泛型 / impl Trait**：编译器按每个具体类型各生成一份代码（单态化）。
//!   零虚调用、可内联，热路径接近 C。代价：代码膨胀、编译慢。
//! - **dyn Trait**：一份代码 + 胖指针虚调用。适合插件、异构列表。
//!
//! 撮合 / 解析 / 哈希走泛型。别在内环 `Box<dyn Fn>`。
//! 承认“编译慢也来自单态化”是加分。

fn add_g<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn show_d(a: &dyn std::fmt::Display) {
    println!("dyn Display: {a}");
}

fn main() {
    // 这里会生成一份 i32 的 add_g、一份 f64 的 add_g
    println!("泛型 i32 = {}", add_g(1, 2));
    println!("泛型 f64 = {}", add_g(1.5, 2.5));

    show_d(&"hi");
    show_d(&42);
}
