// go语言的名言：不要用共享内存来通信，要用通信来共享内存。
// Rust支持通过共享状态来实现并发，但不建议这么用
// Channel类似单所有权：一旦将值的所有权转移至Channel，就无法使用它了
// https://doc.rust-lang.org/book/ch16-03-shared-state.html
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
