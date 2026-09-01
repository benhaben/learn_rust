//! 进阶：async 取消 / select!
//!
//! 运行：`cargo run --bin 25_async_cancel`
//!
//! `tokio::select!` 同时等几路 Future，谁先完成用谁，**其它路被 Drop**。
//! Drop Future = 不再 poll = 取消。
//! 这和内核 `select`/`epoll` 不一样：内核是「谁好处理谁，其余 fd 还在」；
//! 这里没赢的 Future 会析构，这次等待停掉。
//!
//! 这**不会**自动回滚业务副作用。IO 停了，交易所可能已经收到下单。
//! 超时包一层 ≠ 事务。要自己做 client order id + 查询 / 幂等。
//!
//! `async fn` 只是返回 Future；不 poll 就不跑。

async fn do_io() {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("do_io 完成（这行不该出现，因为会被超时取消）");
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_io() => println!("IO 先到"),
        _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
            println!("超时：另一路 Future 被 Drop = 取消");
        }
    }
}
