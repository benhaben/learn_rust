// 闭包会捕获环境变量，占用内存，而函数不会
// 捕获的变量有所有权问题
// 1. 取得所有权：FnOnce 
// 2. 可变借用: FnMut
// 3. 不可变借用：Fn

// 创建闭包是，通过闭包对环境值的使用，rust推断出具体使用哪个trait
// 所有闭包都实现了FnOnce
// 没有移动捕获变量的实现了FnMut
// 无需可变访问捕获变量的闭包实现了Fn


fn main(){
    let x = vec![1,2,3];
    // 在参数列表前使用move关键字可以强制闭包获取所有钱，线程好用
    let equal_to_x = move |z| z == x;
    // println!("{:?}",x);//error value borrowed here after move
    let y = vec![1,2,3];
    assert!(equal_to_x(y))
}

// 记忆化，延迟计算
// struct需要知道所有字段类型，包括闭包，每个闭包都有自己的唯一匿名类型，哪怕他们一样。所以得用泛型

struct Cacher<T>
where T: Fn(u32)->u32{
    calc:T,
    value:Option<u32>, // 只能计算一次，可以用hashmap
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    fn new(calc:T)->Cacher<T>{
        Cacher{
            calc,
            value:None,
        }
    }

    fn value(&mut self,arg:u32)->u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calc)(arg)
                self.value = Some(V);
                v
            }
        }
    }
}