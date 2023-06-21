// standard input output
use std::io;
// random number generator
use rand::Rng;
// standard sorting and ordering
use std::cmp::Ordering;

fn main() {
    // Generate a random number from 1-100
    // This is a local seed generated with current thread and OS
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    
    println!("Guess the number!");

    // infinite loop - play forever!
    loop {
        println!("Please input your guess:");
        // mutable string variable
        let mut guess = String::new();
    
        // standard input - read user input
        io::stdin()
            // read first line of user input
            // & indicates Ref, and single memory allocation
            .read_line(&mut guess)
            // Result itself has an `expect()` method for error handle
            .expect("Failed to read line");
    
        // Parse input str back into u32 int, good for small, positive integers
        let guess: u32 = match guess.trim().parse() {
            // continue with guesses and reexecute loop
            Ok(num) => num,
            // on error, smother exception. Onward!
            Err(_) => continue,
        };

        // use variable in str
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher..."),
            Ordering::Greater => println!("Lower..."),
            // Termination condition!
            Ordering::Equal => {
                println!("WINNER CHICKEN DINNER THINGY");
                // Break terminates the
                break;
            }
        }
    }
}
