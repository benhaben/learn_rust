// unsafe存在的原因
// 静态分析师保守的，可以告诉编译器，自己知道在干啥
// 计算机硬件本身就是不安全的，rust需要对底层硬件编程
// 使用unsafe关键字
// 解引用原始指针
// 调用unsafe函数或方法
// 访问或修改可变的静态变量
// 实现unsafetrait

// 注意：
// unsafe并没有关闭借用检查或停用其他安全检查
// 任何内存安全相关的错误必须停留在unsafe块里
// 尽可能隔离unsafe代码，最好将其封装在安全的抽象里，提供安全API


// 不安全代码的安全抽象

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}


// abi
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main1() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Calling Rust Functions from Other Languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

