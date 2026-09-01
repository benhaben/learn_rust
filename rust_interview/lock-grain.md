# 锁粒度：为什么 map 要 Mutex、Vec 不要

对应：`cargo run --bin 33_lock_grain`。

## 结构

```text
Vec  [ 锁0 | 锁1 | ... | 锁7 ]     ← new 之后不再 push/pop
         └── 每把锁里一台 HashMap   ← bump 改的是这里
```

按 symbol 哈希进第 `i` 片，只锁那一台 map。不同品种常并行；一把全局 `Mutex<HashMap>` 会把多品种串行化。

临界区只改内存，不要锁里 serde / 写盘 / 回调 Python。

## 必须 Mutex 吗

跨线程改同一台 map：要 `Mutex` / `RwLock` / `DashMap` 等。单线程或只读可以不锁。  
本例是多线程改价格，所以每片一把锁。

## 为什么 Vec 不包 Mutex

`Vec` 只做 `shards[i]`：读第 `i` 个槽。长度固定，没人扩容。对 Vec 只有共享读（`&self`）。  
`Mutex<HashMap>` 是 `Sync`，故 `Vec<Mutex<...>>` 多线程拿 `&Books` 下标安全。

运行时还要 `push` 新分片，才要 `Mutex<Vec<...>>`，或一开始定死片数（本例 `new(8)`）。

口条：**锁包会变的数据。map 会变；Vec 只是固定分片目录，只读，不必再锁。**

## Bytes

`bytes::Bytes`：冻结后的引用计数切片，`clone` 不拷字节。转发行情合适；真正落地仍可能 `to_vec`。
