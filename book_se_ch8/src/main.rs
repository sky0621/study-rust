fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    v.push(100);
    v.push(200);
    v.push(300);
    println!("{:?}", v);
    println!("{}", v[1]);
    let vv = v[2];
    println!("{}", vv);
    println!("{}", v[2]);
    let second = v.get(1);
    println!("{:?}", second);
    let none = v.get(100);
    println!("{:?}", none);
    v.push(400);
    println!("{:?}", v);
    let _vv22 = &v[2];
    v.push(500);

    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);
    // v2.push(4);

    let mut v3: Vec<i32> = Vec::new();
    vvv(&mut v3);
    println!("{:?}", v3);

    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];
    v.push(6);

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    for i in v4 {
        println!("{}", i);
    }

    // ----------------------------------------
    let txt = String::from("blue");
    let rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(txt),
        SpreadsheetCell::Float(10.12),
    ];
    for row in rows {
        match row {
            SpreadsheetCell::Int(3) => println!("Int:3"),
            SpreadsheetCell::Text(txt) => println!("Text:blue"),
            SpreadsheetCell::Float(10.12) => println!("Float:10.12"),
            _ => {}
        }
    }
}

fn vvv(v: &mut Vec<i32>) {
    v.push(333);
    v.push(444);
    v.push(555);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
