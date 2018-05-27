use std::io;

fn main() {
    // 「!」付きはマクロ
    println!("Guess the number!");
    println!("Please input your guess.");

    // 「let文」＝変数束縛を作る
    // "guess"という名前の束縛を作り、それを値"String::new()"に束縛する
    // 変数はデフォルトでイミュータブル
    // 「mut」＝ミュータブルな束縛にする
    // 「String」＝伸長可能でUTF-8でエンコードされたテキスト片
    // 「::new()」＝関連関数
    let mut guess = String::new();

    // 
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
