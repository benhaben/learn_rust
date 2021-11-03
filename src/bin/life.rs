// 生命周期
// rust的每个引用都有自己的生命周期。
// 生命周期：引用保持有效的作用域
// 大多数情况：生命周期是隐式的、可被推断的
// 当引用的生命周期可能以不同的方式互相关联时：手动标注生命周期
// 生命周期的主要目标：避免悬垂引用（dangling reference）

// 借用检查器

// 生命周期标注语法
// 生命周期标注描述了多个引用的生命周期关系。但不影响生命周期
// 以'开头，很多开发者用’a,在&符号后面 &'a mut i32

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest(x: & str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn main1() {
    let string1 = String::from("abcd");
    let result: &str;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str()); // error: borrowed value does not live long enough

    // longest() 借用检查器查看函数签名，发现返回值和x,y都是’a生命周期，那么取短的那个，就是string2，所以返回值和string2的生命周期一样
    println!("{}", result);
    let result = longest2(string1.as_str(), string2.as_str());
    println!("{}", result);
}

// 指定生命周期参数的方式依赖于函数所做的事情， 函数返回只和x有关，所以只需要给x指定‘a生命周期即可
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期相同
// 如果返回的引用没有指向任何参数，那么它只能引用函数年内创建的值：
// - 这就是悬垂引用：该值在函数结束时就走出了作用域
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    let ret = String::from("xyz");
    // 所以rust不用于返回悬垂引用，只能返回值，把上面的引用改成String即可编译通过
    // ret.as_str()
    "xxx"
}

// 结构体中生命周期标注
#[derive(Debug)]
struct Apple<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("yin shen");
    let first;
    {
        first = novel.split(" ").next().expect("can not find ' ' ");
    }

    let i = Apple { part: first };

    // first 生命周期比i长

    println!("{:#?}", i.level());
}

// 生命周期的省略
// 1.每个引用都有生命周期
// 2.需要为使用生命周期的函数或者struct指定生命周期参数
// 3.为什么有的函数引用参数没有生命周期呢？因为编译器发现可以自己处理，就不需要加了。未来可能有更多情况不需要加
// 在rust引用分析中所编入的模式称为生命周期省略规则、
// 这些规则无序开发者来指定
// 他们由编译器处理

// 输入，输出生命周期
// 生命周期在函数方法的参数：输入生命周期
// 函数方法返回值：输出生命周期

// // 生命周期省略的三个规则
// 编译器使用3个规则在没有显示标注生命周期的情况下，来确定引用的生命周期
// - 规则1用于输入生命周期
// -规则 2，3应用于输出生命周期
// -如果编译器应用完3个规则之后，仍然有无法确定生命周期的引用-》报错
// - 这些规则使用于fn定义和impl块

// 规则1：每个引用类型参数都有自己的生命周期
// 规则2：如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数
// 规则3：如果有多个输入生命周期参数，但其中一个是&self或&mut self，那么self生命周期会被付给所有的输出生命周期参数

// 假设我们是编译器1
// fn first(s: &str)->&str{}
// fn first<'a>(s: &'a str)->&str{} 规则1
// fn first<'a>(s: &'a str)->&'a str{} 规则2

// 假设我们是编译器2
// fn longest2(x: &str, y:  &str) -> &str {
// fn longest2<'a，'b>(x: &'a str, y:  &'b str) -> &str { 规则2
// 无法计算出返回类型的生命周期，编译器报错

// 假设我们是编译器3 - 方法
impl<'a> Apple<'a> {
    //规则1
    fn level(&self) -> &str {
        " "
    }

    //规则3
    fn level1(&self, a: &str) -> i32 {
        1
    }
}

// 静态生命周期
// ’static是一个特殊的生命周期：整个程序的持续时间
// 所有的字符串字面值都有'static生命周期
// let s:&'static str = "xxxx";
