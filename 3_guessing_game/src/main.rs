// standard input output lib
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess:");

    // mutable string variable
    let mut guess = String::new();

    // standard input
    io::stdin()
        // read first line of user input
        .read_line(&mut guess)
        // error condition
        .expect("Failed to read line");

    // use variable
    println!("You guessed: {guess}");
}
