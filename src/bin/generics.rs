struct Point<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

impl<T, U> Point<T, U> {}

impl Point<i32, u32> {}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {}
