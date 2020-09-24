pub enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    pub fn call(&self) {
        println!("{:?}", self);
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("{}", i);
            Some(i + 1)
        }
    }
}
