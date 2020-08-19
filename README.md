# study-rust

## Ref

https://www.rust-lang.org/ja/learn

### THE BOOKを読もう！
https://doc.rust-lang.org/book/

## Next
https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/ownership.html

## OS
```
$ cat /etc/os-release 
NAME="Ubuntu"
VERSION="18.04.4 LTS (Bionic Beaver)"
```

## rustc
https://www.rust-lang.org/learn/get-started
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$
$ rustc --version
rustc 1.45.2 (d3fb005a3 2020-07-31)
```

## cargo
```
$ cargo --version
cargo 1.45.1 (f242df6ed 2020-07-22)
```

## compile
rustc main.rs

## make project
cargo new prj0x --bin

## build
cargo build
cargo build --release

## run
cargo run

## run with stacktrace
RUST_BACKTRACE=1 cargo run
