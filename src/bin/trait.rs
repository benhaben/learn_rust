// trait类似interface，但有区别
// trait bounds 约束，泛型类型参数指定为实现了特定行为的类型
use ::guessing_game::NewsArticle;
use guessing_game::Summary;
fn main() {
    let article = NewsArticle {
        headline: "xxxx".to_string(),
    };
    println!("{}", article.summarize());
}

// 实现trait约束
// 这个类型或者trait是本地的crate里面定义的，比如为标准库的vec实现一个trait或者定义trait
// 这个是一致性原则，更具体的说是孤儿原则，此规则确保其他人的代码不能破坏你的代码，反之亦然。
// 如果没有这个规则，链各个crate可以为同一个类型实现同一个trait，rust就是不知道用哪个实现

