mod Father {
    pub mod Son {
        pub  fn say() {}
    }
    fn say() {}
}

pub fn out() {
    crate::Father::Son::say(); //绝对路径。大部分人使用这个
    Father::Son::say(); //相对路径
}

// 模块不仅可以组织·代码，还可以定义私有边界
// 如果想把函数或者struct等设为私有，可以将它放到某个模块中
// rust中所有的条目（函数，方法，struct，enum，模块，常量）默认都是私有的
