use std::fmt::Debug;

fn main() {
    // Shared behavior with traits
    let news_article = NewsArticle {
        author: String::from("Nicholas Hazel"),
        headline: String::from("Saving the world, one line at a time"),
        content: String::from("This is a test content for my news article!"),
    };
    let tweet = Tweet {
        username: String::from("sinsys"),
        content: String::from("This is a test tweet for my tweet!"),
        reply: false,
        retweet: false
    };
    println!("{}", news_article.summarize());
    println!("{}, by {}", tweet.summarize(), tweet.summarize_author());

    notify_sugar(&news_article, &tweet);
    notify_plain(&news_article, &news_article);

    // Using Summary trait
    println!("{}", returns_summarizable().summarize());
    println!("{}", returns_summarizable2().summarize());

    // Trait bounds
    let pair1 = Pair::new(42, 52);
    let pair2 = Pair::new(52, 42);
    pair1.cmp_display();
    pair2.cmp_display();
}
#[derive(Debug)]
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
pub trait Summary {
    // No default implementation
    fn summarize_author(&self) -> String;

    // Has default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    // MUST supply non default implementations!
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    // MUST supply non default implementations!
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Optionally, customize default!
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }

}

// This method demonstrates that a return type MUST implement the Summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("the quick brown fox"),
        reply: false,
        retweet: false
    }
}

// This will BREAK! Even though we use the Summary trait,
// we are only allowed to return one type of that trait
// This breaks since we also add in conditional to also
// return a NewsArticle. Implied traits can derive a return type,
// but only ONE.
// fn returns_summarizable_broken(switch: bool) -> impl Summary {
//     if (switch) {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("the quick brown fox"),
//             reply: false,
//             retweet: false
//         }
//     } else {
//         NewsArticle {
//             author: String::from("Nicholas Hazel"),
//             headline: String::from("Saving the world, one line at a time"),
//             content: String::from("This is a test content for my news article!"),
//         }
//     }
// }

// This method demonstrates that a return type MUST implement the Summary trait
fn returns_summarizable2() -> impl Summary {
    NewsArticle {
        author: String::from("Nicholas Hazel"),
        headline: String::from("Saving the world, one line at a time"),
        content: String::from("This is a test content for my news article!"),
    }
}

// Item can be ANY type that implements Summary
pub fn notify_sugar(item: &(impl Summary + Debug), item2: &(impl Summary + Debug)) {
    println!("Item2: {:#?}", item2);
    println!("Breaking news! {}", item.summarize());
}

// Actually syntactic sugar for a trait bound, which is...
// Another caveat is that item and item2 MUST be the SAME type here!
pub fn notify_plain<T: Summary + Debug>(item: &T, item2: &T) {
    println!("Item2: {:#?}", item2);
    println!("Breaking news! {}", item.summarize());
}

// Trait bounds
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair <T> {
    fn new(x:T, y: T) -> Self {
        Self {x,y}
    }
}
// This is a trait bound. Type MUST implement partial ordering and display to console
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest num: {}", self.x)
        } else {
            println!("The largest num: {}", self.y)
        }
    }
}

// Blanket implementations for traits
// Longer lesson required on usage
// impl<T: Display> ToString for T {
//     // implement
// }