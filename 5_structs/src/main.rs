fn main() {
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
    println!("origin: {origin:?}");
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