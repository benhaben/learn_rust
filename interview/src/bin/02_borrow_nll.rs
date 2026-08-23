//! 必考：借用互斥 + NLL（Non-Lexical Lifetimes）
//!
//! 运行：`cargo run --bin 02_borrow_nll`
//!
//! # 规则（必须背）
//!
//! 同一时刻：
//! - 要么有任意多个不可变借 `&T`（只读共享），
//! - 要么有且仅有一个可变借 `&mut T`（独占写），
//! - 不能两者同时存在。
//!
//! **借用不是所有权搬家。** `&T` / `&mut T` 只是“临时查看”，owner 还在。
//! 借用结束前，owner 不能被 move、不能再被可变借。
//!
//! **NLL**：借用的寿命不再死板地绑到花括号，而是到“最后一次使用”就结束。
//! 所以 `let n = s.len(); s.push_str(...)` 能过：`len` 的 `&s` 用完就死了。

fn append_len(s: &mut String) {
    // `s.len()` 需要 `&String`（不可变借）。
    // NLL：这个不可变借在 `n` 算出来之后立刻结束，因为后面不再用那个引用。
    let n = s.len();
    // 这里可以再拿 `&mut s`：没有未结束的 `&s` 和它冲突。
    s.push_str(&n.to_string());
}

fn ok_copy_then_push(v: &mut Vec<i32>) {
    // `v[0]` 对 Copy 类型是“复制出值”，不是借住元素。
    // 复制完成后，对 v 的不可变借结束，可以 push。
    let x = v[0];
    v.push(x);
}

fn _wont_compile_if_uncommented(v: &mut Vec<i32>) {
    // let x = &v[0]; // x 的类型是 &i32，借用一直活到 x 最后一次使用
    // v.push(1);     // push 要 &mut Vec，和 x 重叠 → 编译失败
    // println!("{x}");
    let _ = v;
}

fn main() {
    let mut s = String::from("hi");
    append_len(&mut s);
    println!("append_len 之后: {s}"); // "hi2"

    let mut v = vec![10, 20];
    ok_copy_then_push(&mut v);
    println!("先 Copy 再 push: {v:?}");

    // 循环里“先借元素再 push”是面试最常见挂点：
    // for x in &v {
    //     v.push(*x); // &v 活过整个循环，和 push 的 &mut v 重叠
    // }

    // 修法 1（i32 是 Copy）：先记下长度，按下标复制出值，借在 push 前结束。
    // 只扫“原来的 n 个”，否则一边 push 一边 len 变长会无限循环。
    let n = v.len();
    for i in 0..n {
        let x = v[i];
        v.push(x);
    }
    println!("按下标 Copy 再 push: {v:?}"); // 原来 [10,20,10]，再追加这 3 个 → [10,20,10,10,20,10]

    // 修法 2：先克隆一份快照再遍历。元素不是 Copy（如 String）时用这个。
    // let snap = v.clone();
    // for x in snap {
    //     v.push(x);
    // }

    // 修法 3：标准库一句，把当前内容复制追加到末尾。
    v.extend_from_within(..);
}
