use crate::gui::{Button, Screen, SelectBox};
use book_se_ch17::AveragedCollection;

mod gui;
mod lib;

fn main() {
    // part1()
    part2()
}

fn part2() {
    Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 150,
                label: String::from("次へ"),
            }),
            Box::new(SelectBox {
                width: 80,
                height: 50,
                label: String::from("店舗"),
                options: vec![String::from("Yes"), String::from("No")],
            }),
        ],
    }
    .run()
}

fn part1() {
    let mut ave = AveragedCollection::new();
    println!("{:?}", ave);
    ave.add(30);
    ave.add(60);
    println!("{}", ave.average())
}
