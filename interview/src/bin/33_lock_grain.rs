//! 压轴：锁粒度 / 热路径缓冲（量化加分）
//!
//! 运行：`cargo run --bin 33_lock_grain`
//!
//! 面试往下追：
//! - 订单簿锁按 **symbol / shard** 还是一把全局 Mutex？全局锁把多品种撮合串行化。
//! - 行情缓冲用 `bytes::Bytes` 还是 `Vec<u8>`？Bytes 是引用计数切片，转发不必拷贝；
//!   乱 `clone` 只加计数，但所有权边界要落地时仍会 `to_vec`。
//! - 临界区只改内存结构，不要在锁里 serde / 写盘 / 回调 Python。
//!
//! 本例：按 symbol 哈希进分片锁。

use std::collections::HashMap;
use std::sync::Mutex;

use bytes::Bytes;

struct Books {
    shards: Vec<Mutex<HashMap<String, i64>>>,
}

impl Books {
    fn new(n: usize) -> Self {
        Self {
            shards: (0..n).map(|_| Mutex::new(HashMap::new())).collect(),
        }
    }

    fn bump(&self, sym: &str, px: i64) {
        let i = fxhash(sym) % self.shards.len();
        *self.shards[i]
            .lock()
            .unwrap()
            .entry(sym.to_string())
            .or_insert(0) = px;
    }

    fn get(&self, sym: &str) -> Option<i64> {
        let i = fxhash(sym) % self.shards.len();
        self.shards[i].lock().unwrap().get(sym).copied()
    }
}

fn fxhash(s: &str) -> usize {
    s.bytes()
        .fold(0usize, |h, b| h.wrapping_mul(31).wrapping_add(b as usize))
}

fn main() {
    let books = Books::new(8);
    books.bump("BTCUSDT", 60_000);
    books.bump("ETHUSDT", 3_000);
    println!("BTC = {:?}, ETH = {:?}", books.get("BTCUSDT"), books.get("ETHUSDT"));

    // Bytes：从 Vec 冻结后，clone 不拷贝字节
    let raw = Bytes::from(vec![1, 2, 3, 4]);
    let a = raw.clone();
    let b = raw.slice(1..3);
    println!("Bytes clone 后仍共享缓冲: a={a:?} b={b:?}");
}
