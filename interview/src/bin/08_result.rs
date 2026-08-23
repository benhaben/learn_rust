//! 必考：Result / ? / From
//!
//! 运行：`cargo run --bin 08_result`
//!
//! # `?` 是什么
//!
//! ```text
//! expr?
//! ```
//! 近似于：
//! ```text
//! match expr {
//!     Ok(v) => v,
//!     Err(e) => return Err(From::from(e)),
//! }
//! ```
//! 关键是 **From**：io::Error、ParseIntError 都能转成你的错误类型。
//!
//! # 分工（面试原话）
//!
//! - **库 crate**：自己的 `enum` + `thiserror` + `impl From`，让调用方能 match。
//! - **二进制 / 应用**：`anyhow::Error`，用 `.context()` 加“正在做什么”。
//! - 库不要返回 anyhow，否则应用加不了有类型的上下文、也无法稳定 match。
//!
//! `unwrap` 只留给“走到这里说明程序不变量已经坏了”。

use std::fs;
use std::num::ParseIntError;
use std::path::Path;

use thiserror::Error;

/// 库侧：保留错误种类，好让应用决定怎么展示 / 重试。
#[derive(Debug, Error)]
enum LoadErr {
    #[error("读文件失败: {0}")]
    Io(#[from] std::io::Error),
    #[error("不是整数: {0}")]
    Parse(#[from] ParseIntError),
}

fn load_lib(path: &Path) -> Result<i32, LoadErr> {
    // 第一个 ?：io::Error → LoadErr::Io
    // 第二个 ?：ParseIntError → LoadErr::Parse
    Ok(fs::read_to_string(path)?.trim().parse()?)
}

/// 应用侧：不关心枚举变体，只要能打印一串上下文。
fn load_app(path: &Path) -> Result<i32, anyhow::Error> {
    let n: i32 = fs::read_to_string(path)?.trim().parse()?;
    Ok(n)
}

fn main() -> anyhow::Result<()> {
    let path = std::env::temp_dir().join("rust-interview-n.txt");
    fs::write(&path, "42\n")?;

    println!("库错误类型: {:?}", load_lib(&path));
    println!("应用 anyhow: {}", load_app(&path)?);

    // 触发 Parse
    fs::write(&path, "not-a-number\n")?;
    match load_lib(&path) {
        Ok(n) => println!("不该成功: {n}"),
        Err(e) => println!("预期的库错误: {e}"),
    }
    Ok(())
}
