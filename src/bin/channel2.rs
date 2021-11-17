// rust语言并发特性较少，目前讲的并发特性都来自标准库
// 无需局限于标准库的并发，可以自己实现并发
// 但在rust语言中有两个并发概念
// std::marker::sync,std::marker::Send两个trait

// 实现send trait的类型课可以转移所有权
// rust中几乎搜友的类型都实现了send，但Rc<T>么
// 任何完全又send类型组成的类型也被标记为Send
// 除了原始指针之外，几乎所有的基础类型都是Send

// 实现Sync的类型可以安全的被多个线程引用
// 如果T是Sync，那么&T就是Send - 引用可以被安全的送往另一个线程
// 基础类型都是Sync
// 完全由Sync类型组成的类型也是Sync
// Rc<T>不是synv
// RefCell和Cell<T>家族也不是Sync

// 自己实现不太容易保证安全
// https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html
/*
Implementing Send and Sync Manually Is Unsafe
Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually. As marker traits, they don’t even have any methods to implement. They’re just useful for enforcing invariants related to concurrency.

Manually implementing these traits involves implementing unsafe Rust code. We’ll talk about using unsafe Rust code in Chapter 19; for now, the important information is that building new concurrent types not made up of Send and Sync parts requires careful thought to uphold the safety guarantees. “The Rustonomicon” has more information about these guarantees and how to uphold them.
 */
fn main(){}