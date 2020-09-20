fn main() {
    let g = Get {};
    exec(&g);
}

trait Command {
    fn run(&self);
}

struct Get {}

impl Command for Get {
    fn run(&self) {
        println!("Get!");
    }
}

fn exec(cmd: &dyn Command) {
    cmd.run();
}
