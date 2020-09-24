extern crate tmp;

use tmp::lib1;
use tmp::mlib1::say_mlib1;
use tmp::pkg01::pkg01other::say_pkg01other;
use tmp::pkg01::say_pkg01;

fn main() {
    lib1();
    say_mlib1();
    say_pkg01();
    say_pkg01other();
}
