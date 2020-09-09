use std::fmt;

fn main() {
    // println!("{0}-{1}-{2}-{1}-{0}", 100, 200, 300);

    // println!(
    //     "{} of {:b} people know binary, the other half doesn't",
    //     1, 2
    // );

    // println!("{number:>width$}", number = 1, width = 1);
    // println!("{number:>width$}", number = 2, width = 2);
    // println!("{number:>width$}", number = 3, width = 3);

    // let v = List(vec![1, 2, 3]);
    // println!("{}", v);

    // let x = 1_000_000;
    // println!("{}", x);

    // let tpl = (-100, "文字列", 50, 3.14, 'A', true);
    // println!("{:?}", tpl);
    // println!("{:?}", tpl.0);
    // println!("{:?}", tpl.3);

    // let ary: [u32; 3] = [1, 2, 3];
    // println!("{:?}", ary);
    // let ary2: [f64; 5] = [1.0; 5];
    // println!("{:?}", ary2);

    let ary = [5, 10, 15];
    let ary2 = &ary;
    println!("{:?}", ary);
    println!("{:?}", ary2);
}

// Define a structure named `List` containing a `Vec`.
// `Vec`を含む`List`という名の構造体を定義
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        // `v`を介して`vec`をイテレーションし、同時にカウントを
        // `enumerate`で取得する
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        // 開きっぱなしのブラケットを閉じて、`fmt::Result`の値を返す。
        write!(f, "]")
    }
}
