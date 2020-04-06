
01 hanoi
========

1. `cargo new` 連 git 都會起始化，附帶 `.gitignore`

2. `trait Copy` 的用法跟 JS 語法的 primitive type 一樣

3. `char` 據說是 4 bytes，代表一個 unicode

4. `if` 後面的判斷式不用括號，`rustc` 會提醒

5. `[println!]` 聽說是巨集，用 `{}` 做代換

[println!]: https://doc.rust-lang.org/std/macro.println.html


02 three color flags
====================

1. 預設 immutable，會改變的值要加 mut

2. 有 byte string `b"String"` 這 `byte` `b'c'` 兩種 literal，會成為 `&'_static[u8;n]` 和 `u8`，可以比較像 C 的版本使用，但是是 immutable ... 要轉成 Vec 再轉成 slice: `b"S".to_vec().as_mut_slice()`。只是這樣就會用到 heap 了 :Q

3. 從 slice 拿長度 : `.len()`

4. Vec / slice 有自帶 [swap]

5. 如果 `println!("{:?}", b"TEST")`，會變成 `[84, 69, 83, 84]` ... 好像要再轉回 String 再印，但是 `String::from_utf8()` 回傳的是 `Result<String, FromUtf8Error>`，這邊可以直接 `.unwrap()`

6. `{:?}` 需要 `trait Debug` ... 效果類似 golang 的 `%v` ?

[swap]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap


看起來最應該寫的版本:

```rust
    // ver 1
    let mut flags = b"BRWBWR".to_vec();
    arrange(flags.as_mut_slice());
    println!("{:?}", String::from_utf8(flags).unwrap());
```

所以... String 有個 [as_bytes_mut] 但是是 unsafe

[as_bytes_mut]: https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes_mut

```rust
    // ver 2
    let mut flags = String::from("BRWBWR");
    let mut flags = unsafe { flags.as_bytes_mut() };
    arrange(flags);
    println!("{:?}", std::str::from_utf8_mut(&mut flags).unwrap());
```

byte string literal 怎樣找不到變成 mut 的方法... 但是用 array literal 可以

```rust
    // ver 3
    let flags : &mut [u8;6] = &mut [b'B', b'R', b'W', b'B', b'W', b'R'];
    arrange(flags);
    println!("{:?}", std::str::from_utf8_mut(&mut flags[..]).unwrap());
```

04 maze
=======

1. 沒找到文件，但是可以用 nested use + as:

```rust
use MazeCell::{
    Wall as W,
    PathTaken as T,
    PathUntaken as U,
};
```

2. slice 的單位一定要是固定大小，所以好像只能用 `Vec` ?

3. 支援用 `{}` 印出一個 type 的方法: implement [std::fmt::Display] trait:

[std::fmt::Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html

```rust
use std::fmt;
impl fmt::Display for MazeCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            W => write!(f, "W"),
            T => write!(f, "T"),
            U => write!(f, "U"),
        }
    }
}

4. 可以用 `for v in vecotr.iter()` 來 traverse `Vec`
```
