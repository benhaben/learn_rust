// 需要在heap上分配数据，这些数据被程序多个部分读取（只读），但在编译时无法确定哪个部分最后使用完这些数据
// Rc<T>只能用于单线程场景
// Rc::clone(&a)函数：增加引用计数
// Rc::strong_count(&a)：获得引用计数
// Rc::weak_count函数

// pub enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::Cons;
// use crate::List::Nil;
// fn main() {
//     let a = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a)); // error

// }

// 改成下面这样

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::Cons;
use crate::List::Nil;
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a)); // error
}
