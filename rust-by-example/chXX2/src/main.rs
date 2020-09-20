fn main() {
    let tr = Request { kind: Text {} };
    println!("{:?}", tr.kind.record());

    let vr = Request { kind: Voice {} };
    println!("{:?}", vr.kind.record());

    let mr = Request { kind: Movie {} };
    println!("{:?}", mr.kind.record());
}

#[derive(Debug)]
struct Request<T>
where
    T: Recorder,
{
    kind: T,
}

trait Recorder {
    fn record(&self) {
        println!("default");
    }
}

#[derive(Debug)]
struct Text {}

impl Recorder for Text {
    // fn record(&self) {
    //     println!("text...");
    // }
}

#[derive(Debug)]
struct Voice {}

impl Recorder for Voice {
    fn record(&self) {
        println!("voice...");
    }
}

#[derive(Debug)]
struct Movie {}

impl Recorder for Movie {
    fn record(&self) {
        println!("movie...");
    }
}
