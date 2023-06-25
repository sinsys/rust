

fn main() {
    struct_demo();
    // PROGRAM BEGIN
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square sp px",
        rect1.area()
    );
    println!("{}", area(&rect1));
    let rect2 = Rectangle {
        width: 5,
        height: 10
    };
    let rect3 = Rectangle {
        width: 500,
        height: 10
    };
    let rect4 = Rectangle {
        width: 10,
        height: 500
    };
    println!("can_hold rect2?: {}", rect1.can_hold(&rect2));
    println!("can_hold rect3?: {}", rect1.can_hold(&rect3));
    println!("can_hold rect4?: {}", rect1.can_hold(&rect4));
    let square1 = Rectangle::square(42);
    println!("I made a square!: {square1:#?}");
}
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn struct_demo() {
    let mut user1 = build_user(
        String::from("foo"),
        String::from("foo@bar.com")
    );
    let user2 = User {
        email: String::from("different@domain.com"),
        ..user1
    };
    println!("user2: {user2:?}");
    user1.username = String::from("Uh oh, I changed...");
    let username = user1.username;
    let email = user1.email;
    let active = user1.active;
    let sign_in_count = user1.sign_in_count;
    println!("username in struct is: [{username}]");
    println!("email in struct is: [{email}]");
    println!("active in struct is: [{active}]");
    println!("sign_in_count in struct is: [{sign_in_count}]");
    let black = Color(0, 0, 0);
    println!("black: {black:?}");

    let origin = Point(0, 0, 0);
    let some_struct = Rectangle {
        height: 10,
        width: 20
    };
    // PRINT STRUCTS
    // No formatting
    println!("origin: {origin:?}");
    // Pretty formatting
    println!("println!: {:#?}", some_struct);
    // DEBUG PRINT STRUCT
    // Pretty formatting => stderr
    dbg!(some_struct);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width &&
        self.height > rect.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}