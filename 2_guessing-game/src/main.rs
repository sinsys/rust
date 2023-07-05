use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    Game::run_game();
}

pub struct Game {
    secret: u32
}

impl Game {
    pub fn new() -> Game {
        let secret = rand::thread_rng()
            .gen_range(1..=100);
        Game { secret }
    }

    pub fn guess(&self) -> u32 {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim().parse::<u32>()
            .expect("Could not read guess!");
        guess
    }

    pub fn eval_guess(&self, guess: u32) -> bool {
        println!("You guessed: {guess}");
        match guess.cmp(&self.secret) {
            Ordering::Less => {
                println!("Higher...");
                false
            },
            Ordering::Greater => {
                println!("Lower...");
                false
            },
            Ordering::Equal => {
                println!("WINNER CHICKEN DINNER THINGY");
                true
            }
        }
    }

    pub fn run_game() {
        println!("Guess the number!");
        let game = Game::new();
        loop {
            let guess = game.guess();
            let eval = game.eval_guess(guess);
            if eval {
                break;
            } else {
                continue;
            }
        }
    }
}