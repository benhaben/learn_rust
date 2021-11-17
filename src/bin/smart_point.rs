// 标准库中常见的智能指针
// Box<T>: 在heap内存上分配值，实现了Deref trait和drop trait
// Rc<T>:启用多重所有权的引用计数类型
// Ref<T> 和 RefMut<T>,通过RefCell<T>访问:在运行时而不是编译时强制借用规则的类型

// 内部可变模式：不可变类型暴露出克修改其内部值的API
// 循环引用：如何泄露内存，如何防止其发生

// Box<T>使用场景：
// 编译时，某类型大小无法确定。但使用该类型时，上下文却需要知道它的确切大小
// 当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制。
fn box1() {
    let b = Box::new(5);
    //释放栈上指针和堆上内存
}

// 使用box赋能递归类型
// 在编译时，rust需要知道一个类型所占的空间大小，而递归类型大小无法在编译时确定。
// Cons List 来自Lisp语言的一种数据结构，包含当前项目值和下一个元素（本身类型），链表
// ConsList不是Rust常用的，Vec常用

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::Cons;
use crate::List::Nil;
fn box2() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        //自动释放处理，rust不允许手动调用，但可以用全局的drop函数
        println!("xxxxdrop")
    }
}

fn hello(name: &str) {
    println!("hello, {}", name);
}

fn box3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
    // *(y.deref())

    // deref coercion,编译时完成
    let m = MyBox::new(String::from("Rust"));
    // &m -> &MyBox<String>
    // deref &String
    // deref &str
    hello(&m);
}
fn main() {
    box3();
}
