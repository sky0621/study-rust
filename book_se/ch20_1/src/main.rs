use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connected!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    thread::sleep(Duration::from_secs(5));
    let mut f = File::open("hello.html").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let res = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
