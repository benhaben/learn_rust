// 3a
// arrange
// act
// assert

// 测试函数需要使用test属性（attribute）进行标注
// - attribute就是一段rust代码的元数据
// 在函数上加#[test],可以把函数变成测试函数
// 测试函数panic！就是失败
// 每个测试运行在线程里面

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn test1() {
    let a = 1;
    assert_eq!(a, 1, "我是自定义信息");
}

#[test]
fn test2() {
    let a = 2;
    assert_eq!(add_two(a), 1, "我是自定义信息-{}", "哈哈");
}

// 是否发生的panic
#[test]
#[should_panic(expected = "期望发生的panic文字")]
fn test3() {
    let a = 2;
    assert_eq!(add_two(a), 1, "没有发生panic,失败");
}

mod test_mod {
    // 测试中使用Result，无需panic

    pub fn add_two1(a: i32) -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn test4() {
        let a = 2;
        add_two1(a);
    }
}
fn main() {}

// 改变cargo test的行为
// 默认并行所有测试,确保不互相依赖和共享状态
// cargo test --help
// cargo test -- --help

// 顺序执行
// cargo test -- --test-threads=1

// 默认成功不会打印println的输出
// cargo test -- --show-output

// 运行测试的子集,函数名字,模块名称
// cargo test test1

// 忽略测试 #[ignore]

// rust对测试的分类
// 单元测试，可以访问私有接口
// 集成测试

// 单元测试
// #[cfg(test)]
// 只有运行cargo test才会编译和运行

// 集成测试在不同的目录，他不需要#[cfg(test)]

#[cfg(test)]
mod test {
    // 可以测试私有函数
    fn add_two3(a: i32) -> Result<(), String> {
        Ok(())
    }
    #[test]
    fn test5() {
        let a = 2;
        add_two3(a);
    }
}
