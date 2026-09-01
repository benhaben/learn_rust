//! 进阶：GAT / async fn in trait
//!
//! 运行：`cargo run --bin 27_gats_async_trait`
//!
//! **GAT（generic associated type）**：关联类型自己也能带寿命 / 泛型。
//! 经典动机：lending iterator——`next` 返回的引用借的是 `self`，下一次 `next` 会作废上一次。
//! 普通 `Iterator::Item` 做不到这一点（Item 不能依赖 `self` 的寿命）。
//!
//! **async fn in trait**：现在稳定。以前要写成
//! `fn get(&self) -> impl Future<Output = String> + '_`，或 `#[async_trait]` 装箱。
//! `dyn Trait` 里若含 async，通常仍要装箱的 Future。
//!
//! `main` 里 Windows 循环和 Dummy.get 没有关系，两个考点拼在一个文件。
//! `'s` 是源字符串；`Item<'_>` 的寿命是这次 `&mut self`（省略，等于 Item<'a>）。
//! 不是 next 完就不能 next，是还拿着返回值时不能再 next。

trait LendingIter {
    type Item<'a>
    where
        Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}

struct Windows<'s> {
    s: &'s str,
    i: usize,
}

impl<'s> LendingIter for Windows<'s> {
    type Item<'a>
        = &'a str
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.i + 2 > self.s.len() {
            return None;
        }
        let piece = &self.s[self.i..self.i + 2];
        self.i += 1;
        Some(piece)
    }
}

trait Fetch {
    async fn get(&self, id: u64) -> String;
}

struct Dummy;

impl Fetch for Dummy {
    async fn get(&self, id: u64) -> String {
        format!("row-{id}")
    }
}

#[tokio::main]
async fn main() {
    let mut w = Windows { s: "abcd", i: 0 };
    while let Some(p) = w.next() {
        print!("{p} ");
    }
    println!();
    println!("{}", Dummy.get(7).await);
}
