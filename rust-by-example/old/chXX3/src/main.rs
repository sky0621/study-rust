fn main() {
    let args = vec!["abc"];
    let g = Get {};
    exec(g, args);

    let args2 = vec!["def"];
    let l = List {};
    exec(l, args2);

    let g2 = Get {};
    let l2 = List {};
    let mut v: Vec<Command> = vec![];
    v.push(Get {});
    v.push(List {});
}

fn exec<T: Command>(c: T, args: Vec<&str>) {
    println!("{:?}", c.run(args).unwrap());
}

trait Command {
    fn run(&self, args: Vec<&str>) -> Result<String, i32>;
}

struct Get {}

impl Get {
    fn getget() {
        println!("getget")
    }
}

impl Command for Get {
    fn run(&self, args: Vec<&str>) -> Result<String, i32> {
        println!("Get!");
        match args.len() {
            0 => Err(1),
            _ => Ok(format!("Get ok: {:?}", args)),
        }
    }
}

struct List {}

impl List {
    fn listlist() {
        println!("listlist")
    }
}

impl Command for List {
    fn run(&self, args: Vec<&str>) -> Result<String, i32> {
        println!("List!");
        match args.len() {
            0 => Err(1),
            _ => Ok(format!("List ok: {:?}", args)),
        }
    }
}
