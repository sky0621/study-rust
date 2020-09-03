# study-rust

## Ref

https://doc.rust-jp.rs/book/second-edition/

### Cargo

https://doc.rust-lang.org/cargo/

### crates.io

https://crates.io/

### Tool

#### clippy

https://github.com/rust-lang/rust-clippy

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

- Rustにはnull値が存在しない
- Rustで要素のリストがある場合はほとんどの場合、`Vec<T>`を使用するのがよりよい選択
- 参照の規則
```
　・任意のタイミングで、『一つの可変参照』か『不変な参照いくつでも』のどちらかを行える。
　・『参照は常に有効』でなければならない。
```
- Box<T>, Rc<T>, RefCell<T>を選択する理由
```
　・Rc<T>は、同じデータに複数の所有者を持たせてくれる
　・Box<T>とRefCell<T>は単独の所有者
```
```
　・Box<T>では、不変借用も可変借用もコンパイル時に精査できる
　・Rc<T>では不変借用のみがコンパイル時に精査できる
　・RefCell<T>では、不変借用も可変借用も実行時に精査される
```
```
　・RefCell<T>は実行時に精査される可変借用を許可するので、
　　RefCell<T>が不変でも、 RefCell<T>内の値を可変化できる。
```

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

##### スタックとヒープ
<pre>
スタックは高速
データを取得する場所を探す必要が絶対にない
スタック上のデータは全て既知の固定サイズでなければならない
</pre>
<pre>
ヒープにデータを置く時、OSはヒープ上に十分な大きさの空の領域を見つけ、使用中にし、ポインタを返します。
ヒープへのデータアクセスは低速
ポインタを追って目的の場所に到達しなければならないから
</pre>

#### 4.2 参照と借用
- 関数の引数に参照を取ることを借用と呼びます。
- 変数が標準で不変なのと全く同様に、**参照も不変**
- ダングリングポインタとは、 他人に渡されてしまった可能性のあるメモリを指すポインタのこと

#### 4.3 スライス
- 所有権のない別のデータ型は、スライス

#### 5.3 メソッド
- 最初の引数は必ずselfになり、これはメソッドが呼び出されている構造体インスタンスを表します
- 関連関数＝implブロック内にselfを引数に取らない関数

#### 6.2 match
- Rustにおけるマッチは、包括的

#### 7 モジュール
- モジュールとは、関数や型定義を含む名前空間

#### 8.2 文字列型
- StringはVec<u8>のラッパ

<pre>
let mut s = String::from("hello");
s.push_str(", world!");   // push_str()関数は、リテラルをStringに付け加える
println!("{}", s);        // これは`hello, world!`と出力する
</pre>

##### 文字列リテラル
- 文字列リテラルでは、文字列の値はプログラムにハードコードされます。 
- 文字列リテラルが不変である
- 文字列リテラルの場合、中身はコンパイル時に判明しているので、テキストは最終的なバイナリファイルに直接ハードコードされます。

#### 9.3 panic!
- コードが悪い状態(何らかの前提、保証、契約、不変性が破られたこと)に陥る可能性があるときにパニックさせるのは、推奨されること
- しかし、悪い状態に達したとき、それでもpanic!呼び出しをするよりも、 Resultを返すほうがより適切

#### 10.1 ジェネリクス
- 単相化(monomorphization)は、コンパイル時に使用されている具体的な型を入れることで、 ジェネリックなコードを特定のコードに変換する過程のこと
- Rustでは、ジェネリックなコードを各インスタンスで型を指定したコードにコンパイルするので、 ジェネリクスを使用することに対して実行時コストを払うことはありません。コードを実行すると、 それぞれの定義を手作業で複製した時のように振る舞います。

#### 10.2 トレイト
- 動的型付け言語では、型が実装しない型のメソッドを呼び出せば、実行時にエラーが出るでしょう。 しかし、Rustはこの種のエラーをコンパイル時に移したので、コードが動かせるようにさえなる以前に問題を修正することを強制されるのです。

#### 10.3 ライフタイム
- ライフタイムとは、その参照が有効になるスコープのこと
- 参照は全てライフタイム(その参照が有効になるスコープのこと)を保持する
- ライフタイムの主な目的は、ダングリング参照(参照するつもりだったデータ以外のデータを参照)を回避すること
- 関数から参照を返す際、戻り値型のライフタイム引数は、引数のうちどれかのライフタイム引数と一致する必要があります。

##### ダングリングポインタ
<pre>
無効なメモリ領域を指すポインタはダングリングポインタ（dangling pointer）と呼ばれる。
とりわけ、本来有効だったメモリ領域が解放処理などによって無効化されたにもかかわらず、そのメモリ領域を参照し続けているポインタのことを、ダングリングポインタと呼ぶ。
</pre>

##### ライフタイム省略規則
- 参照である各引数は、独自のライフタイム引数を得るというものです。換言すれば、 1引数の関数は、1つのライフタイム引数を得るということ
- 1つだけ入力ライフタイム引数があるなら、そのライフタイムが全ての出力ライフタイム引数に代入されるというもの
- 複数の入力ライフタイム引数があるけれども、メソッドなのでそのうちの一つが&selfや&mut selfだったら、 selfのライフタイムが全出力ライフタイム引数に代入されるというもの

##### 静的ライフタイム
- 'staticであり、これはプログラム全体の期間を示します。 文字列リテラルは全て'staticライフタイム

#### 12 コマンドラインプログラム
##### バイナリプロジェクトの責任の分離
###### 工程
- プログラムをmain.rsとlib.rsに分け、ロジックをlib.rsに移動する。
- コマンドライン引数の解析ロジックが小規模な限り、main.rsに置いても良い。
- コマンドライン引数の解析ロジックが複雑化の様相を呈し始めたら、main.rsから抽出してlib.rsに移動する。

###### main関数に残る責任
- 引数の値でコマンドライン引数の解析ロジックを呼び出す
- 他のあらゆる設定を行う
- lib.rsのrun関数を呼び出す
- runがエラーを返した時に処理する

#### 13.1 クロージャ
- Rustのクロージャは、変数に保存したり、引数として他の関数に渡すことのできる匿名関数
- クロージャは、3つの方法で環境から値をキャプチャでき、この方法は関数が引数を取れる3つの方法に直に対応します: 所有権を奪う、可変で借用する、不変で借用するです。

#### 13.2 イテレータ
- Rustにおいて、イテレータは怠惰(イテレータを使い込んで消費するメソッドを呼ぶまで何の効果もない)

#### 14.1 プロファイル
- devプロファイル(cargo buildコマンドを実行したときに使用)は、開発中に役に立つデフォルト設定
- releaseプロファイル(cargo build --releaseコマンドを実行したときに使用)は、 リリース用の設定

#### 14.3 ワークスペース

<pre>
$ cargo new adder --bin
$ cargo new addone --lib
</pre>

```
book_se_ch14_cargo
  |- adder
  |    |- src
  |    |    |- main.rs ③
  |    |- Cargo.toml ②
  |- addone
  |    |- src
  |    |    |- lib.rs ④
  |    |- Cargo.toml
  |- Cargo.lock
  |- Cargo.toml ①
```

①
<pre>
[workspace]

members = [
    "adder",
    "addone",
]
</pre>

②
<pre>
[package]
name = "adder"
　〜〜

[dependencies]

addone = { path = "../addone" }
</pre>

③
<pre>
extern crate addone;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, addone::add_one(num));
}
</pre>

④
<pre>
pub fn add_one(x: i32) -> i32 {
    x + 1
}
</pre>

#### 15 スマートポインタ
##### 15.1 Box
- Boxはヒープのデータを指す（ヒープデータへのポインタがスタックに格納される）

##### 15.4 Rc
- Rc::cloneの実装は、多くの型のclone実装のように、全てのデータのディープコピーをすることではありません。
- Rc::cloneの呼び出しは、参照カウントをインクリメントするだけであり、時間はかかりません。

##### 15.5 RefCell/内部可変性
- 内部可変性は、そのデータへの不変参照がある時でさえもデータを可変化できるRustでのデザインパターン
- コードが借用規則に従っているとプログラマは確証を得ているが、コンパイラがそれを理解し保証することができない時に RefCell<T>型は有用
