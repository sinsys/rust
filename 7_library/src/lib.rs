// These flags suppress warnings
#![allow(dead_code)]
#![allow(unused_variables)]
// IMPORTS
pub use crate::front_of_house::hosting;
// Import syntax
// use rand::{Rng, CryptoRng, ErrorKind::Transient};
// use std::io::*;

// MODULES
// This is inferred using an identical folder name
mod front_of_house;

// We create this module inline
mod back_of_house {
    // Enums are powerful grouping types
    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfast {
        // Public properties can be edited!
        pub toast: String,
        seasonal_fruit: String,
    }

    // This adds methods to the Breakfast struct
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            let result = Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
            println!("summer");
            result
            
        }
    }

    // This is available to back_of_house only
    fn fix_incorrect_order() {
        // local func
        cook_order();
        // super func (get from higher scope)
        super::serve_order();
        println!("fix_incorrect_order");
    }

    fn cook_order() {
        println!("cook_order");
    }
}

// FUNCTIONS
fn serve_order() {
    println!("serve_order");
}

// This is the main, public function to be used for the lib
pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();
    // relative
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // We can edit an order!
    let mut meal = self::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    // Ordering different appetizers
    let order1 = self::back_of_house::Appetizer::Soup;
    let order2 = self::back_of_house::Appetizer::Salad;
    println!("eat_at_restaurant");

}
