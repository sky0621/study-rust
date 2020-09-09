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

fn main() {
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
