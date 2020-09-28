fn main() {
    let dv = Dove {};
    let dc = Duck {};
    dv.tweet();
    dv.tweet_twice();
    dv.shout();
    dc.tweet();
    dc.tweet_twice();
    dc.shout();

    println!();

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dv), Box::new(dc)];
    for b in bird_vec {
        b.tweet_twice();
    }
}

trait Tweet {
    fn tweet(&self);
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }
    fn shout(&self) {
        println!("Uhooooooo");
    }
}

struct Dove;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

struct Duck;

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}
