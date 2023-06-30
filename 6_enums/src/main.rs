enum Message {
    Write(String)
}
impl Message {
    fn call(&self) {
        println!("hmm");
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("LUCKKKKKYYY!!!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let message = Message::Write(
        String::from("hello...")
    );
    message.call();
    // A match is a pattern where we can eval "optional" values
    let some_num: Option<i8> = Some(5);
    match some_num {
        None => println!("PANIC"),
        Some(val) => println!("some_num is: {val}"),
    }
    println!("P: {}\nN: {}\nD: {}\nQ:{}",
        value_in_cents(Coin::Penny),
        value_in_cents(Coin::Nickel),
        value_in_cents(Coin::Dime),
        value_in_cents(Coin::Quarter)
    );
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("I R 3"),
        _ => println!("{:?}", some_value)
    }
    if let Some(3) = some_value {
        println!("I R 3");
    }
}
