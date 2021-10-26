//vector
fn main() {
    // let v: Vec<i32>= Vec::new();
    let mut v = vec![1, 2, 3]; //宏创建

    // 写入
    // let mut v = Vec::new();
    v.push(1);

    //读取
    // 索引，get
    let third = &v[2];

    match v.get(2) {
        Some(third) => println!("{}", third),
        None => println!("no"),
    }

    //不能同时借用为可变和不可变的
    // let first = &v[0];
    // v.push(6);
    // println!("{}",first);
    //why?vec连续放的，可能内存不够，可能就会移动

    // 遍历vector
    for i in &v {
        println!("{}", i)
    }

    // 遍历修改
    for i in &mut v {
        *i += 50; //解引用
    }

    // 利用枚举往vector存贮多种类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
    ];

    //如果类型实在太多，可以使用trait对象
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
