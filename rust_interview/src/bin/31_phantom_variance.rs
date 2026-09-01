//! 压轴：PhantomData / 型变
//!
//! 运行：`cargo run --bin 31_phantom_variance`
//!
//! `PhantomData<T>` 是**零大小**标记：告诉编译器“请把我当成持有 T”，
//! 从而影响 drop check 和型变。运行期什么也不占。
//!
//! # 型变（variance）一句话
//!
//! - **协变**（covariant）：`&'long T` 能当 `&'short T` 用。寿命变短更安全。
//! - **逆变**（contravariant）：`fn(&'short T)` 能当 `fn(&'long T)` 用。
//!   能处理短寿命的函数，更能处理长寿命输入。
//! - **不变**（invariant）：`&mut T`、`Cell<T>`。既不能当更长也不能当更短，
//!   否则会通过可变别名写出 use-after-free。
//!
//! 裸指针 `*mut T` 对 T **不变**，且不表达所有权。自己写 Box 要加
//! `PhantomData<T>` 恢复“我拥有 T”（drop check + 协变）。
//!
//! | 标记 | 对 T 的型变 |
//! |---|---|
//! | `PhantomData<T>` / `PhantomData<&'a T>` | 协变 |
//! | `PhantomData<fn(T)>` | 逆变 |
//! | `PhantomData<*mut T>` | 不变 |

use std::marker::PhantomData;

/// 教学用假 Box：指针本身不告诉编译器我们拥有 T。
struct MyBox<T> {
    ptr: *mut T,
    _own: PhantomData<T>,
}

impl<T> MyBox<T> {
    fn new(v: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(v)),
            _own: PhantomData,
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // SAFETY: ptr 来自 Box::into_raw，且只 drop 一次。
        unsafe {
            drop(Box::from_raw(self.ptr));
        }
    }
}

/// 用 fn 指针把寿命做成近似不变，防止 `'long` 和 `'short` 乱转。
struct Inv<'a, T> {
    _m: PhantomData<fn(&'a T) -> &'a T>,
}

fn main() {
    let b = MyBox::new(7u32);
    unsafe {
        println!("MyBox 里 = {}", *b.ptr);
    }
    drop(b); // Drop 会释放，没有 PhantomData 时 dropck 可能漏掉 T 里的引用

    let _inv: Inv<'static, i32> = Inv { _m: PhantomData };
    println!("PhantomData 是零大小: {}", std::mem::size_of::<PhantomData<String>>());
}
