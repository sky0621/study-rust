use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // let ... 変数宣言。immutable
    // mut ... mutable
    // String::new() ... static method
    let mut guess = String::new();

    // read_line() ... return 'Result<usize>'
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // {} ... プレースホルダー
    println!("You guessed: {}", guess);
}
