use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // default ... i32
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        // let ... 変数宣言。immutable
        // mut ... mutable
        // String::new() ... static method
        let mut guess = String::new();

        // read_line() ... return 'Result<usize>'
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing
        // unsigned 32 bit
        // parse() ... return Result
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} ... プレースホルダー
        println!("You guessed: {}", guess);

        // match 式
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
