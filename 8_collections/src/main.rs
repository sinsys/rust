// These flags suppress warnings
#![allow(dead_code)]
#![allow(unused_variables)]
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    demo_el_iteration();
    vector_enums();
    strings();
    hashmaps();
    hashmaps_two();
    
}

fn demo_el_iteration() {
    println!("! BEGIN demo_el_iteration !");
    // Create a new array;
    let a = [1,2,3,4,5];
    let mut vec_with_new: Vec<i32> = Vec::new();
    vec_with_new.push(1);
    vec_with_new.push(2);
    vec_with_new.push(3);
    let second = &vec_with_new[2];
    vec_with_new.push(42);


    let vec_from_vals = vec![1,2,3,4,5];
    let mut i = 0;
    let max = &vec_from_vals.len();

    // My trerrible while loop
    while i < max + 2 {
        match vec_from_vals.get(i) {
            Some(el) => println!("El: {}", el),
            None => println!("NOTHING!"),
        };
        i = i + 1;
    }

    // Increase all vals in vector by 50
    let mut mutable_vec = vec![1,2,3,4,5];
    for i in &mut mutable_vec {
        *i += 50;
        println!("{}", i);
    }
    println!("! END demo_el_iteration !\n");
}

fn vector_enums() {
    println!("! BEGIN vector_nums !");

    // Enum defines potential types available in collection
    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    // Create a vector containing all sheet cells
    let row = vec![
        SheetCell::Int(42),
        SheetCell::Float(3.14159),
        SheetCell::Text(String::from("I Win!")),
    ];

    // Iterate over the vector and print all Integers
    for (i, _cell) in row.iter().enumerate() {
        match &row[i] {
            SheetCell::Int(i) => println!("{}", i),
            _ => println!("Not an integer!")
        };
    }
    println!("! END vector_nums !\n");
}

fn strings() {
    println!("! BEGIN strings !");
    // Collection of utf-8 encoded bytes
    // UTF-8 requires ASCII for string encoding
    // ASCII can only represent 128 characters
    // Unicode is a universal character set and represents all charset!
    // First 128 symbols are ASCII!
    // UTF-8 is variable width. Each can be 1,2,3, or 4 bytes!

    // Methods of string creation
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let mut s4 = String::from("initial contents");
    println!("s1: {}\ns2: {}\ns3: {}\ns4: {}", s1, s2, s3, s4);

    s4.push_str("bar");
    s4.push('!');
    println!("s4: {}", s4); // "s4: initial contentsbar!"

    let s5 = s4 + &s3;
    println!("s5: {}", s5); // "s5: initial contentsbar!initial contents"

    let s6 = format!("{}{}", s2, s3);
    println!("s6: {}", s6); // "s6: initial contentsinitial contents"

    let s7 = String::from("नमस्ते");
    // Cannot do this! Characters are 1-4 bytes
    // let c1 = s7[0];
    for b in s7.bytes() {
        println!("byte: {}", b);
    }
    for c in s7.chars() {
        println!("char: {}", c);
    }
    for g in s7.graphemes(true) {
        println!("grapheme: {}", g);
    }
    println!("! END strings !\n");
}

fn hashmaps() {
    println!("! START hashmaps !");
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();
    scores.insert(&blue, 10);
    scores.insert(&yellow, 10);
    scores.insert(&blue, 10);
    scores.insert(&blue, 50);
    scores.entry(&yellow).or_insert(100);
    scores.entry(&yellow).or_insert(10);

    let home_score = scores.get(&blue);
    let visitor_score = scores.get(&yellow);

    println!("Home: {:?}\nVisitor: {:?}\n", home_score, visitor_score);
    println!("{:#?}", scores);

    println!("! END hashmaps !\n");
}

fn hashmaps_two() {
    println!("! START hashmaps_two !");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
    println!("! END hashmaps_two !\n");
}
