extern crate ca;

use ca::domain::item as ditem;
use ca::usecase::item as uitem;

fn main() {
    println!("start");
    uitem::exec();
    ditem::exec();
}
