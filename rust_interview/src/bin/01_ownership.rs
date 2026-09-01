//! 必考：Ownership / move / Copy
//!
//! 运行：`cargo run --bin 01_ownership`
//!
//! # 三个词先分清
//!
//! - **所有权 (ownership)**：每个值在同一时刻只有一个 owner。owner 离开作用域时，值被 Drop（释放）。
//! - **move**：赋值 / 传参时，所有权从**右边（来源）搬到左边（新绑定）**。
//!   `let b = a;` 里 a 在右、b 在左：a 立刻失效，只剩 b 能用。再用不让编译。
//!   实现上通常是拷贝栈上那一小段（指针、长度、容量），堆数据不拷贝。
//! - **Copy**：实现了 `Copy` 的类型，赋值时是“隐式按位复制”，新旧绑定都有效。
//!   `i32`、`bool`、`char`、`&T` 是 Copy；`String` / `Vec` / `Box` 不是。
//! - **Clone**：显式复制。`Copy` 一定能 `Clone`，但 `Clone` 可能很贵（堆分配）。
//! - **Drop**：值的析构函数。离开作用域时编译器插入一次 `drop(值)`。
//!   栈格子本身确实会随函数返回自动废掉（栈指针一挪就没了），这不叫 Drop。
//!   Drop 管的是格子里还抓着的**堆内存 / 锁 / 文件**。`i32` 只有栈上 4 字节，
//!   没有堆，所以几乎感觉不到 Drop；`String` 必须 Drop，否则堆上的 `"hi"` 泄漏。
//!   move 之后旧名字不再 Drop，避免两个 owner 把同一块堆释放两次（double-free）。
//!   `Copy` 和自定义 `Drop` 不能同时存在：复制后两边都会 Drop，又会 double-free。

fn take(s: String) {
    // `s` 成为这段堆字符串的新 owner。
    // 函数结束时 `s` 被 Drop：堆上的字节被释放。
    println!("take 拿到了所有权: {s}");
}

fn main() {
    // ---------- move：String 不是 Copy ----------
    // String 在栈上是 (ptr, len, cap)，真正的字节在堆上。
    // 如果允许 `let b = a` 之后 a、b 都能用，两个 owner 会 double-free。
    // 所以编译器选择 move：a 失效，只剩 b 能释放。
    let a = String::from("hi");
    take(a);
    // println!("{a}"); // 打开会报错：value borrowed here after move

    // ---------- Copy：i32 只活在栈上 ----------
    // 复制一份 4 字节整数没有 double-free 问题，所以是 Copy。
    let n = 3;
    let m = n; // 不是 move，是复制。n 仍然有效。
    println!("Copy 之后两边都能用: n={n}, m={m}");

    // ---------- 部分移动 ----------
    // 结构体里只要有一个字段被 move 走，整个结构体就不能再当整体用。
    struct Pair {
        name: String,
        age: u32, // u32 是 Copy
    }
    let p = Pair {
        name: String::from("alice"),
        age: 20,
    };
    let name = p.name; // name: String 被 move
    let age = p.age; // age: Copy，相当于复制
    println!("部分移动后: name={name}, age={age}");
    // println!("{}", p.name); // 不行：name 已 move
    // let _q = p;            // 不行：整体已被部分移动

    // ---------- Clone 是显式、可能昂贵的复制 ----------
    let left = String::from("tick");
    let right = left.clone(); // 堆上再分配一份字节
    println!("Clone 后两个都有效: {left} / {right}");
}
