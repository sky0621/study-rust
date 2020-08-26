fn main() {
    let user1 = User {
        username: String::from("Taro"),
        email: String::from("taro@example.com"),
        sign_in_count: 3,
        active: true,
    };
    // println!("{}", user1);
    let username = String::from("Jiro");
    let email = String::from("jiro@example.com");
    let mut user2 = build_user(username, email);
    user2.active = false;

    let user3 = User {
        username: String::from("Saburo"),
        email: String::from("saburo@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);

    // --------------------------------------------
    let width1 = 30;
    let height1 = 50;
    println!("{}", area(width1, height1));

    let rect1 = (30, 50);
    println!("{}", area2(rect1));

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rec1);
    println!("{:#?}", rec1);
    println!("{}", area3(&rec1));

    println!("rec1.area: {}", rec1.area());

    let mut rec2 = Rectangle {
        width: 30,
        height: 50,
    };
    rec2.setWidth(20);
    println!("rec2.area: {}", rec2.area());

    let rec3 = Rectangle::square(100);
    println!("{:#?}", rec3);
    println!("{}", rec3.area());
}

// 構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// タプル構造体
struct Color(i32, i32, i32);

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn setWidth(&mut self, w: u32) {
        self.width = w;
    }

    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
