
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