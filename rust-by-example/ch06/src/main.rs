use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct FormPerson {
    name: String,
    age: u32,
}

impl From<DbPerson> for FormPerson {
    fn from(p: DbPerson) -> Self {
        FormPerson {
            name: p.name,
            age: p.age,
        }
    }
}

impl Into<DbPerson> for FormPerson {
    fn into(self) -> DbPerson {
        DbPerson {
            name: self.name,
            age: self.age,
        }
    }
}

#[derive(Debug)]
struct DbPerson {
    name: String,
    age: u32,
}

fn part1() {
    let d = DbPerson {
        name: String::from("Taro"),
        age: 10,
    };
    println!("{:#?}", d);
    println!();

    let f = FormPerson::from(d);
    println!("{:#?}", f);
    println!();

    let d2: DbPerson = FormPerson::into(f);
    println!("{:#?}", d2);
}

#[derive(Debug, PartialEq)]
struct Trump(u32);

impl TryFrom<u32> for Trump {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1..=13 => Ok(Trump(value)),
            _ => Err(()),
        }
    }
}

fn part2() {
    println!("{:?}", Trump::try_from(0));
    println!("{:?}", Trump::try_from(1));
    println!("{:?}", Trump::try_from(13));
    println!("{:?}", Trump::try_from(14));

    let n: Result<Trump, ()> = 10.try_into();
    assert_eq!(Trump::try_from(10), n);
}

struct BloodType {
    t: &'static str,
}

impl Display for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Blood type is {}", self.t)
    }
}

fn part3() {
    let b = BloodType { t: "B" };
    println!("{}", b);

    let nine: i32 = "9".parse().unwrap();
    println!("{}", nine);
}

fn main() {
    part3();
}
