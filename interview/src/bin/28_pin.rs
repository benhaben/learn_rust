//! 压轴：Pin / Unpin
//!
//! 运行：`cargo run --bin 28_pin`
//!
//! # 为什么有 Pin
//!
//! 自引用结构：字段 A 是 `String`，字段 B 是指向 A 内部的指针。
//! 一旦整个结构被 **move**（栈上挪地址），B 就悬空。
//!
//! `async` 生成的状态机经常这样：局部变量还活着，后面的 `.await` 引用它。
//! 所以 `Future::poll` 吃 `Pin<&mut Self>`：承诺“被 poll 之后不要再挪这个值”。
//!
//! # Pin 不是锁
//!
//! - 大多数类型自动 impl **Unpin**（“挪我也没事”）。对 Unpin，`Pin<&mut T>` 几乎没约束。
//! - 含 `PhantomPinned` 的类型是 `!Unpin`。对它们 `get_mut` / `mem::replace` 是 UB。
//!   PhantomPinned 是类型定义上的标签，不是另一种 Pin。
//! - `Box::pin(t)` 把值放到堆上并钉住；地址在 Box 释放前不变。
//!
//! `dummy_cx` / RawWaker 是假 Context，好调用 poll；业务用 tokio，不必手写。

use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 教学用迷你 Future：接口和标准库一样，poll 必须经过 Pin。
trait MiniFut {
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>;
}

struct Done;

impl MiniFut for Done {
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Ready(())
    }
}

/// 这个类型告诉编译器：不要当我 Unpin。
struct Immobile {
    _pin: PhantomPinned,
}

fn pin_box<T>(t: T) -> Pin<Box<T>> {
    Box::pin(t)
}

fn main() {
    let mut f = pin_box(Done);
    // Pin<Box<T>> 可以转成 Pin<&mut T>
    let mut cx = dummy_cx();
    match MiniFut::poll(f.as_mut(), &mut cx) {
        Poll::Ready(()) => println!("Pin 住的 Future poll 一次就 Ready"),
        Poll::Pending => unreachable!(),
    }

    let _immobile = pin_box(Immobile {
        _pin: PhantomPinned,
    });
    println!("PhantomPinned 的值只能待在 Pin 后面，不能再 move 出来");
}

/// 造一个能过类型检查的 Context（不会真的去调度）。
fn dummy_cx() -> Context<'static> {
    use std::ptr;
    use std::task::{RawWaker, RawWakerVTable, Waker};

    unsafe fn clone(_: *const ()) -> RawWaker {
        dummy_raw()
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    fn dummy_raw() -> RawWaker {
        RawWaker::new(
            ptr::null(),
            &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
        )
    }

    let waker = unsafe { Waker::from_raw(dummy_raw()) };
    Context::from_waker(Box::leak(Box::new(waker)))
}

// 压住未使用警告：我们用 MiniFut 的 impl，也展示一下标准 Future 的签名长什么样
fn _std_signature<F: Future<Output = ()>>(f: F) -> F {
    f
}
