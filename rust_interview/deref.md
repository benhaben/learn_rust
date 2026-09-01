# Deref 强制转换

对应：`cargo run --bin 23_deref`。`String` → `&str` 的实现见 [string-str-dst.md](string-str-dst.md)。

## 不是「基本类型都有 Deref」

`i32`、`bool` 没有再往里剥的一层。`1` 就是 `1`。

`Deref` 是给**包装类型**的：用起来像里面的 `T`。

| 你手里有 | `Target` | 所以可以 |
|---|---|---|
| `&String` | `str` | 传给 `fn takes_str(s: &str)` |
| `&Vec<T>` | `[T]` | 当成 `&[T]` |
| `Box<T>` / `Rc<T>` | `T` | `*b` 得到里面的值 |

只沿引用往里走，不会把 `&str` 变成 `String`。要可变目标再 `DerefMut`。

方法解析也会自动解：`s.len()` 其实是 `str::len(&s)`。自定义 `Deref` 别玩花的。

口条：**`Deref` 是包装的「看起来像里面」；不是基本类型的标配。**
