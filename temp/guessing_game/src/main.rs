use std::io;

// デフォルトで読み込む標準ライブラリ＝「プレリュード」

// 返り値なし＝空のタプル
fn main() {
    // 「!」付きはマクロ
    println!("Guess the number!");
    println!("Please input your guess.");

    // 「let」＝変数束縛＝デフォルトでイミュータブル
    // 「mut」＝ミュータブル
    // 「String」＝文字列型
    // 「::」＝スタティックメソッド
    let mut guess = String::new();

    // 「io::stdin()」＝ターミナルの標準入力へのハンドルを返す
    // 「read_line()」は「&mut String」を引数に取る。そして「io::Result」を返す
    // 「io::Result.expect()」
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // 「{}」＝プレースホルダ
    println!("You guessed: {}", guess);
}
