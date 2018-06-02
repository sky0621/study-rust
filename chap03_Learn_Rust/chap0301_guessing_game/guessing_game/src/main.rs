extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    // 「!」付きはマクロ
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // 「let文」＝変数束縛を作る
    // "guess"という名前の束縛を作り、それを値"String::new()"に束縛する
    // 変数はデフォルトでイミュータブル
    // 「mut」＝ミュータブルな束縛にする
    // 「String」＝伸長可能でUTF-8でエンコードされたテキスト片
    // 「::new()」＝関連関数
    let mut guess = String::new();

    // 「stdin()」＝ターミナルへの標準入力
    // 「read_line()」＝「io::Result」を返す
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
