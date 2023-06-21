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
        // & indicates Ref, and single memory allocation
        .read_line(&mut guess)
        // Result itself has an `expect()` method we can call
        //
        .expect("Failed to read line");

    // use variable (also demonstrate expansion)
    // empty {} expand args in order
    println!("You guessed: {guess} and expanded number is: {}", 5 + 5);
}
