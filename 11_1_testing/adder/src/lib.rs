pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
pub struct Guess {
    val: i32
}
impl Guess {
    pub fn new(val: i32) -> Guess {
        if val < 1 {
            panic!("guess must be above 0");
        } else if val > 100 {
            panic!("guess must be below 101");
        }
        Guess { val }
    }
}
#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}
impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rect { w: 10, h: 10 };
        let smaller = Rect { w: 1, h: 1 };
        let result = larger.can_hold(&smaller);
        assert_eq!(result, true);
    }
    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rect { w: 10, h: 10 };
        let smaller = Rect { w: 1, h: 1 };
        let result = smaller.can_hold(&larger);
        assert_eq!(result, false);
    }
    #[test]
    fn add_two_adds_two() {
        let result = add_two(1);
        assert_eq!(result, 1 + 2);
    }
    #[test]
    fn add_two_adds_properly() {
        let result = add_two(1);
        assert_ne!(result, 1);
    }
    #[test]
    fn greeting_contains_name() {
        let name = String::from("Nicholas");
        let result = greeting(&name);
        assert!(
            result.contains("Nicholas"),
            // Custom failure message
            "Greeting did not contain name, value was `{}`",
            // Placeholders in msg
            result
        );
    }
    #[test]
    #[should_panic(expected = "guess must be below 101")]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected = "guess must be above 0")]
    fn less_than_1() {
        Guess::new(-42);
    }
    // Result type!
    #[test]
    fn it_really_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }
}
