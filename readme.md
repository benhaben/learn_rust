# 通过release profile来自定义构建
```
[profile.dev]
opt-level = 0
```
# 在https://crates.io/上发布库
```
cargo publish

cargo yank
```

# 通过workspace组织大工程
共享target目录，防止反复的编译

在cargo.toml里面定义 workspace
```
[workspace]
members=[工作空间名字]
```

参考workspace_test工程

# 从https://crates.io来安装库

```
cargo install

```

~/.cargo/bin/

# 使用自定义命令扩展cargo

```
如果$path中的某个二进制是cargo-something，你可以像子命令一样运行 
cargo something
cargo list

```