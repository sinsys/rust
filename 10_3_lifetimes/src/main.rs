// These flags suppress warnings
#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    examples();
    let ref1 = String::from("x");
    let ref2 = String::from("y");
    let announcement = 42;
    longest_with_an_announcement(&ref1, &ref2, announcement);
}

fn examples() {
    // dangling_ref();
    proper_ref();
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    // All good baby!
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest is: {}", result);

    // STILL GOOD BABY!
    let string3 = String::from("abcd");
    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest is: {}", result2);
    }

    // Uh oh...
    let string5 = String::from("abcd");     // ----------+-- 'a
    {                                               //           |
        let string6 = String::from("xyz");  // -+-- 'b   |
        let result3 = longest(                //  |        |
        string5.as_str(),                        //  |        |
            string6.as_str(),                    //  |        |
        );                                          //  |        |
    }                                               // -+        |
    // result3 has lifetime inaccessible here!      //           |
    // println!("The longest is: {}", result3);     // - 'b Not available!!!

    good_lifetime_ref(&string3, &string5);

    let sentence = String::from("The quick brown fox jumped.");
    let first_word = first_word(&sentence);
    println!("first_word: {}", first_word);

    // String literals are stored in binary, so they survive the program
    let static_lifetime: &'static str = "I have a static lifetime!";
}
// fn dangling_ref() {                     // ----------+-- 'a
//     let r;                              //           |
//     {                                   //           |
//         let x = 5;                      // -+-- 'b   |
//         r = &x;                         //  |        |
//     }                                   // -+        |
//     // Now that this scope closes,      //           |
//     // &x is no longer valid!           //           |
//     // It is a dangling ref             //           |
//     println!("r: {}", r);               //           |
// }                                       // ----------+

fn proper_ref() {                          // ----------+-- 'b
    let r;                           //           |
    let x = 5;                        // -+-- 'a   |
    r = &x;                                //  |        |
    println!("r: {}", r);                  //  |        |
}                                          // ----------+

// X can have different lifetime than y! How does
// compiler infer ref as return value?
// We don't know exact lifetime of x OR y. It could
// be called from ANYwhere!
//
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
//
// missing lifetime specifier
// this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `x` or `y`

// Fix with Generic Lifetime Annotations!
// &i32         // a reference
// &'a i32      // a ref with an explicit lifetime
// &'a mut i32  // a mutable ref with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn bad_lifetime_ref<'a>(x: &'a str, y: &str) -> &'a str {
//     let result = String::from("really long string!");
//     result.as_str() // Problem!!! This Lifetime won't be avail!
// }
// fn bad_lifetime_ref<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string!");
//     result.as_str() // Problem!!! This Lifetime won't be avail!
//     // cannot return reference to local variable `result`
//     // returns a reference to data owned by the current function
// }

// Here we transfer ownership, so the internal creation of String is OK!
fn good_lifetime_ref<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string!");
    result
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    // Works fine, but is not necessary
    // fn return_part(&'a self, announcement: &str) -> &'a str {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention plez: {}", announcement);
        self.part
    }
}
fn lifetime_in_struct() {
    let novel = String::from("Book title");
    let first_sentence = novel.split('.').next()
        .expect("Can't find something");
    // This would break if first_sentence was out of scope!
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

// Returns a ref to the first word in a sentence
fn first_word<'a>(s: &'a str) -> &'a str {
    // Break into iterable
    let bytes = s.as_bytes();
    // Iterate over the bytes
    for (i, &item) in bytes.iter().enumerate() {
        // If the byte is equal to a space...
        if item == b' ' {
            // return a reference to a slice of the word
            return &s[0..i];
        }
    }
    // return a reference to a slice of the first word
    &s[..]
}

// Why does this work though?
// 
fn first_word_no_lifetime(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Example of ALL of chapter 10!
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}