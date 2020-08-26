# study-rust

## Ref

https://doc.rust-jp.rs/book/second-edition/

### Old
https://www.rust-lang.org/ja/learn
https://doc.rust-lang.org/book/

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

## attention

#### 3.2 データ型
- Rustの配列は、 固定長(一度宣言されたら、サイズを伸ばすことも縮めることもできません。)

#### 3.3 関数
- Rustの関数と変数の命名規則は、スネークケース(訳注: some_variableのような命名規則)を使うのが慣例
- 関数シグニチャにおいて、各仮引数の型を宣言しなければなりません。
- 関数本体は、文が並び、最後に式を置くか文を置くという形で形成
- 文とは、なんらかの動作をして値を返さない命令です。 式は結果値に評価されます。

#### 3.5 制御フロー
- 論理値以外の値が、自動的に論理値に変換されることはありません

#### 4.1 所有権
- Rustの各値は、所有者と呼ばれる変数と対応している。
- いかなる時も所有者は一つである。
- 所有者がスコープから外れたら、値は破棄される。

#### 4.2 参照と借用
- 関数の引数に参照を取ることを借用と呼びます。
- 変数が標準で不変なのと全く同様に、**参照も不変**
- ダングリングポインタとは、 他人に渡されてしまった可能性のあるメモリを指すポインタのこと

#### 4.3 スライス
- 所有権のない別のデータ型は、スライス
