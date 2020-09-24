fn main() {
    let c1 = CustomSmartPointer {
        data: String::from("first"),
    };
    println!("{:?}", c1);
    {
        let c2 = CustomSmartPointer {
            data: String::from("second"),
        };
        println!("{:?}", c2);
    }
    drop(c1);
    println!("end");
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping ... {}", self.data);
    }
}
