fn main() {
    let mut s = String::new();
    println!("{:?}", s);

    let data = "This is a data.";
    println!("{:?}", data);
    let s2 = data.to_string();
    println!("{:?}", s2);

    let s3 = &s2;
    println!("{:?}", s3);
    println!("{:?}", s2);

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");
    let ss4 = format!("{}-{}-{}", ss1, ss2, ss3);
    println!("{}", ss4);
}
