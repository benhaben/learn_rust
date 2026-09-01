# Result：二选一、`?` 对齐的是 E

## 怎么实现的

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

两种状态**只能选一个**，不是两个字段并排，也不是共享一份数据。`T`、`E` 是类型参数：成功装什么、失败装什么。内存像带 tag 的 union / `std::variant`，空间取 T 和 E 较大者。

`Option<T>` 同理：`None | Some(T)`，失败那边没有值。

## `?` 对齐 E，不是对齐整个 Result

```text
s.parse()     Result< i32,      ParseIntError >
函数返回      Result< Vec<i32>, ParseIntError >
              Ok 不同            Err 的 E 相同
```

`?`：`Ok(值)` 把值拿出来继续写；`Err(e)` 变成 `return Err(e)`（或 `From::from`）。**不会**把 `Ok(1)` 变成 `Ok(vec![1])`。

```rust
fn parse_ids(ids: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut out = Vec::new();
    for s in ids {
        out.push(s.parse()?); // 成功：i32 推进 Vec；失败：return Err(e)
    }
    Ok(out)                   // 只有这里才出现函数的 Ok(Vec)
}
```

`push` 不返回 Result。函数返回 Result 是因为：失败 `?` 提前 `return Err`，成功最后 `Ok(out)`。

`Result<i32, E>` 和 `Result<Vec<i32>, E>` 的 `Err` 都是同一个 `E`，所以 `return Err(e)` 合法。失败时不交 Vec。

一行版：`ids.iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()`，第一个 `Err` 变成整个 `Err`。

口条：**失败只交 E，成功才交 T。** `?` 交 E 并退出；`T`（Vec）只在最后的 `Ok` 里。
