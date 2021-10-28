
1.  test目录，创建集成测试
2. tests目录下的每个测试文件都是一个单独的crate
3. 需要将测试库导入
4. 无需标注#[cfg(test)]，tests目录被特殊对待，只有cargo test，才会编译tests目录下的文件

如何运行指定的集成测试：
- cargo test 函数名
- cargo test --test 文件名。比如：cargo test --test i_test

如果要创建帮助函数在集成测试，要建一个文件夹

如果项目是binary crate，只有src/main.rs，没有src/lib.rs:
- 不能再tests目录下创建集成测试
- 无法把main.rs的函数导入作用域
- 因为只有library crate才能暴露函数给其他crate用
- binary crate意味着独立运行，main里面一般是胶水代码，不用测试了