//! 进阶：对象安全——哪些 trait 能写成 `dyn Trait`
//!
//! 运行：`cargo run --bin 15_object_safety`
//!
//! # 想解决什么问题
//!
//! C++ 里可以「基类指针指向子类」：运行时再决定调谁的虚函数。
//! Rust 没有继承，对应写法是 `dyn Trait`：只要几个类型都实现了同一个 trait，
//! 就可以装进同一种指针里，例如 `Box<dyn Work>`。调用方法时再查表，决定跳到哪个实现。
//!
//! # 为什么必须写成 `Box<dyn Work>`，不能写 `Box<Work>`
//!
//! `Box<T>` 的 `T` 必须是类型（这块内存长什么样）。
//! `Work` 只是 trait，一份契约，规定有哪些方法，**本身不是一种数据布局**。
//! 不存在「一个 Work 值」，所以也没有 `Box<Work>`。
//!
//! `dyn Work` 才是类型，意思是：这里有一个实现了 Work 的值，但具体是 Ping
//! 还是别的，已经擦掉了。不同实现体积可以不同，所以 `dyn Work` 是 DST
//! （编译期不知道多大），必须用指针拎着：`Box<dyn Work>` 或 `&dyn Work`。
//!
//! 这个指针比普通 `Box<Ping>` 多带一张表，叫**胖指针**：
//! `(指向数据的指针, vtable)`。vtable 是函数指针表，记录这个具体类型的
//! `run`、`clone_box` 在哪个地址。`w.run()` 时编译器不知道里面是谁，
//! 只能查 vtable。老版本曾允许省略 `dyn`，容易和普通类型搞混，现在必须写上。
//!
//! # 什么叫对象安全
//!
//! 要当 `dyn Work` 用，编译器必须在编译时就为这个 trait 造好一张**有限、统一**的 vtable。
//! 每条进表的方法，都必须能在这种情况下调用：只知道「这是个 Work」，
//! 不知道具体类型叫什么、占多少字节。
//!
//! 做不到的方法，会让**整个 trait** 都不能写成 `dyn Trait`。这叫不是对象安全。
//!
//! 两种最常见的违规：
//!
//! 1. 方法返回 `Self`（或按值接收 `Self`）。
//!    例如 `clone(&self) -> Self`：返回值在栈上要留多大？取决于具体类型。
//!    调用方手里只有 `dyn Work`，接不住「大小未知的 Self」。
//!    所以标准库的 `Clone` 不能写成 `Box<dyn Clone>`（下面 main 里有反例）。
//!
//! 2. 方法自己再带一个泛型参数，例如 `fn helper<T>(&self, v: T)`。
//!    每一种 `T`（`i32`、`String`、…）都是一条不同的函数。
//!    编译 `dyn Work` 时不知道你会传入哪种 `T`，vtable 没法预留无穷多项。
//!
//! # 本文件怎么绕开
//!
//! 克隆：不返回 `Self`，返回 `Box<dyn Work>`。盒子本身大小固定（一个胖指针），
//! 里面具体是谁，调用方不必知道。
//!
//! 泛型方法：加上 `where Self: Sized`。`Sized` 的意思是「编译期知道 `Self` 多大」。
//! `Ping` 满足；`dyn Work` 是 DST，不满足。于是这条方法只存在于具体类型上，
//! 不进 vtable，`dyn Work` 上等于没有它。其余方法仍然对象安全，`Box<dyn Work>` 能写。
//! 没有这句的话，光是 `helper<T>` 就会让整个 `Work` 不能当 `dyn` 用。
//!
//! 口条：`dyn` 靠一张有限的 vtable。返回 `Self`、方法再泛型，表造不出来。
//! `Self: Sized` 是开关：这条方法只给具体类型，不要放进 `dyn`。

trait Work {
    fn run(&self);

    /// 克隆成「又一个 trait 对象」，不把具体类型（比如 Ping）暴露给调用方。
    fn clone_box(&self) -> Box<dyn Work>;

    /// 泛型方法。`where Self: Sized` 把它从 vtable 摘掉，否则整个 trait 不能 dyn。
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
        // 新盒子里仍是 Ping，但对外类型是 dyn Work。
        Box::new(Ping)
    }
}

fn main() {
    // Box::new(Ping) 先是 Box<Ping>（普通指针，知道里面就是 Ping）。
    // 赋给 Box<dyn Work> 时，编译器补上 Ping 的 vtable，变成胖指针。
    let w: Box<dyn Work> = Box::new(Ping);
    w.run();

    let w2 = w.clone_box();
    w2.run();

    // w.helper(1);
    // 编译失败：w 的类型是 dyn Work，这条方法被 Self: Sized 摘掉了，vtable 里没有。

    Ping.helper(1); // Self 是 Ping，大小已知，不走 vtable，可以调。

    // let _c: Box<dyn Clone> = Box::new(1);
    // 编译失败：Clone::clone 返回 Self，造不出统一 vtable。
    // 想克隆 trait 对象，像上面一样自己写 clone_box。
}
