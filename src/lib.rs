use std::fmt::{Debug, Display};

mod Father;

pub fn out() {
    crate::Father::Son::say(); //绝对路径。大部分人使用这个
    Father::Son::say(); //相对路径
}

// 模块不仅可以组织·代码，还可以定义私有边界
// 如果想把函数或者struct等设为私有，可以将它放到某个模块中
// rust中所有的条目（函数，方法，struct，enum，模块，常量）默认都是私有的

// use 用来将路径引入到模块中
// 对函数一般引用到父模块
// struct引用到他本身
// as关键字可以给引入的路径指定别名
// use std::io::Result as ioResult;
// 使用use导入的路径是默认私有的。可以使用pub use来导出
// pub use std::io::Result as ioResult;

// 使用外部包
// 1、cargo.toml添加依赖包， https://crates.io/
// 2、use将特定条目引入作用域

//切换国内源
// 进入.cargo
// touch config
// 贴上源 https://cargo.budshome.com/reference/source-replacement.html
// std也是外部包，但不需要引用

// 也可以这样
// use::std::{
//     cmp::Ordering,
//     io
// }

// use::std::{
//      self,
//     cmp::Ordering,
//     io
// }

// use::std*, 一般不这样用，测试的时候可能会导入所有测试模块

// 将模块的内容移动到其他文件
// 1. 模块定义时，如果模块后面是; 而不是代码块：
//     - Rust会从与模块同名的文件中加载内容
//     - 模块树的结构不会变化
// 随着模块变大，该技术可以让你把模块的内容移动到其他文件

pub trait Summary {
    fn summarize(&self) -> String {
        /// 默认实现
        println!("meiyoua");
        "xxxx".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}

//使用trait约束
pub fn notify(item: impl Summary + Display) {
    println!("news:{}", item.summarize());
}

// 这个更好
pub fn notify1<T: Summary + Display>(item: T) {
    println!("news:{}", item.summarize());
}

// 语法where, 约束太长怎么办
pub fn notify2<T: Summary + Display, U: Clone + Debug>(item: T, b: U) {
    println!("news:{}", item.summarize());
}

pub fn notify3<T, U>(item: T, b: U)
where
    T: Summary + Display,
    U: Clone + Debug,
{
    println!("news:{}", item.summarize());
}

// trait实现返回类型.但必须只能返回同一种类型
pub fn notifyR() -> impl Summary{
    NewsArticle{
        headline:"xxxx".to_string()
    }
}