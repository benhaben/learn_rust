//! 必考：Send / Sync（筛人题）
//!
//! 运行：`cargo run --bin 05_send_sync`
//!
//! # 定义（用自己的话讲）
//!
//! - **Send**：这个值的**所有权**可以搬到另一个线程。`thread::spawn(move || x)` 要求 `x: Send`。
//! - **Sync**：`&T` 可以安全地给多个线程同时拿，等价于 `&T: Send`。
//!
//! 自动 impl：编译器按字段推。你几乎从不手写 impl Send（手写是 unsafe）。
//!
//! | 类型 | Send | Sync | 原因 |
//! |---|---|---|---|
//! | i32 / String | 是 | 是 | 没有别名可变 |
//! | Rc\<T\> | 否 | 否 | 引用计数非原子，两个线程 ++ 会丢计数 |
//! | Arc\<T\> | T: Send+Sync | 同左 | 计数原子；内容仍必须可共享 |
//! | Cell\<T\> | T: Send | 否 | `&Cell` 就能 set，多线程会数据竞争 |
//! | RefCell\<T\> | T: Send | 否 | 运行期借检查不是线程安全 |
//! | Mutex\<T\> | T: Send | T: Send | 锁把 `&Mutex` 变成内部可变 |
//! | *const T / *mut T | 否 | 否 | 别名与释放由你保证 |

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Mutex;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<i32>();
    assert_sync::<i32>();
    assert_send::<String>();
    assert_sync::<String>();

    // Rc：两个线程同时 clone 会把非原子计数写坏
    // assert_send::<Rc<i32>>();
    // assert_sync::<Rc<i32>>();
    let _rc = Rc::new(1);

    // Cell：通过共享引用就能改内部，所以不是 Sync
    // assert_sync::<Cell<i32>>();
    let _cell = Cell::new(0);

    // Mutex：多个线程可以同时拿 &Mutex，lock 时才互斥。
    // 条件是 T: Send（锁里的值可能被另一个线程拿出来）。
    assert_send::<Mutex<i32>>();
    assert_sync::<Mutex<i32>>();

    println!("Send/Sync 的静态断言都过了。打开注释掉的那几行会编译失败。");
}
