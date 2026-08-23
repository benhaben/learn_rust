//! 必考：Box / Rc / Arc
//!
//! 运行：`cargo run --bin 04_box_rc_arc`
//!
//! | 类型 | 所有权 | 线程 | 可变 |
//! |---|---|---|---|
//! | `Box<T>` | 唯一 | 可以 Send（若 T: Send） | 直接 `&mut` |
//! | `Rc<T>` | 共享，非原子计数 | 单线程，不是 Send/Sync | 要再包 `RefCell` |
//! | `Arc<T>` | 共享，原子计数 | 跨线程 | 要再包 `Mutex` / `RwLock` |
//!
//! - **Box**：值放到堆上，栈上只留一个指针。用来打破递归类型的无限大小，或做 `dyn Trait`。
//! - **Rc::clone**：只加引用计数，不拷贝 T。循环引用会泄漏，用 `Weak` 打破。
//! - **Arc** 只解决“谁拥有”，不解决“谁能改”。要改再加锁。

use std::rc::Rc;
use std::sync::Arc;

/// 递归 enum 如果直接写 `Cons(i32, List)`，编译器算不出大小（里面还套 List）。
/// `Box<List>` 大小是一个指针，整型 enum 大小就确定了。
enum List {
    Nil,
    Cons(i32, Box<List>),
}

impl List {
    fn from_slice(xs: &[i32]) -> List {
        xs.iter()
            .rev()
            .fold(List::Nil, |tail, &x| List::Cons(x, Box::new(tail)))
    }

    fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons(x, tail) => x + tail.sum(),
        }
    }
}

fn main() {
    let list = List::from_slice(&[1, 2, 3]);
    println!("Box 递归链表 sum = {}", list.sum());

    // Rc：单线程共享只读（或再包 RefCell 做内部可变）
    let a = Rc::new(1);
    let b = Rc::clone(&a); // 计数 2，不是拷贝那个 i32 的“新堆对象语义”
    println!("Rc 计数 = {}, 值 = {}", Rc::strong_count(&a), *b);

    // Arc：可以送进别的线程（这个文件先不 spawn，见 06）
    let c = Arc::new(1);
    let d = Arc::clone(&c);
    println!("Arc 计数 = {}, 值 = {}", Arc::strong_count(&c), *d);

    // Rc 不是 Send，不能 thread::spawn(move || *a)。
    // Arc 可变还要 Mutex，见 06_arc_mutex。
}
