fn main() {
  println!("\n!!! --- OWNERSHIP --- !!!\n");
  demo1();
  demo2();
  demo3();

  println!("\n!!! --- REFERENCE --- !!!\n");
  demo4();
  demo5();
  demo6();
  demo7();
  demo8();

  println!("\n!!! --- SLICE --- !!!\n");
  demo9();
  demo10();
  demo11();
  demo12();
}

fn demo1() {
  println!("\n-START DEMO 1---");
  let s1 = String::from("s1"); // s1 comes into scope
  println!("s1 is in scope [{s1}]");

  println!("s1 is moved to [takes_ownership(s1)]");
  takes_ownership(s1);
  // s1's value moves into the function...
  // ...and so is no longer valid here
  
  let i1 = 5; // i1 comes into scope
  println!("i1 is in scope [{i1}]");

  makes_copy(i1); // i1 would move into the function,
  println!("i1 is moved to [makes_copy(i1)].");  // but i32 is Copy. No memory released.
  println!("i1 is still available since i32 makes copies, while Strings transfer. i1 is [{i1}]");
  println!("-END DEMO 1-----\n");
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("[takes_ownership] has value [{}]", some_string);
  println!("[takes_ownership] drops [{}]", some_string);
}
// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("[makes_copy] has value [{}]", some_integer);
}
// Here, some_integer goes out of scope. Nothing special happens.

fn demo2() {
  println!("\n-START DEMO 2---");
  // This method produces a string and returns it, giving ownership
  let s2 = gives_ownership();
  println!("s2 is in scope [{s2}]");

  // This directly creates a string into scope
  let s3 = String::from("s3");
  println!("s3 is in scope [{s3}]");

  // Transfers ownership of string externally and receives it back
  println!("s2 will be used as input to [takes_and_gives_back(s2)]");
  let s4 = takes_and_gives_back(s2);
  println!("s4 is set to ref to s2 [{s4}]");
  println!("-END DEMO 2-----\n");
}
// Here, s4 goes out of scope and is dropped.
// s3 was moved, so nothing happens.
// s2 goes out of scope and is dropped.

fn gives_ownership() -> String {
  let some_string = String::from("gives_ownership_str"); // some_string comes into scope
  println!("[gives_ownership] created string value [{some_string}]");
  some_string // some_string is returned
}

fn takes_and_gives_back(a_string: String) -> String {
  println!("[takes_and_gives_back] Accepted [{a_string}] and directly returns [{a_string}]");
  a_string
}

fn demo3() {
  println!("\n-START DEMO 3---");
  let s5 = String::from("s5");
  println!("s5 is in scope [{s5}]");

  let (s5_returned, len) = calculate_length(s5);
  println!("The length of [{}] is [{}].", s5_returned, len);
  println!("-END DEMO 3-----\n");
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String
  // We return a tuple that contains the memory of the String as well!
  (s, length)
}

fn demo4() {
    println!("\n-START DEMO 4---");
    let s6 = String::from("s6");
    let len = calculate_length_ref(&s6);
    println!("The length of [{}] is [{}]", s6, len);
    println!("-END DEMO 4-----\n");
}

fn calculate_length_ref(s: &String) -> usize { // s is a reference to a String
  s.len()
}
// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

// This is a demonstration of:
// - A string gets created in memory
// - A reference is passed off instead of value
// - External method modifies str by reference
fn demo5() {
    println!("\n-START DEMO 5---");
    let mut s7 = String::from("s7");
    println!("s7 is in scope [{s7}]");
    change(&mut s7);
    println!("s7 has been modified! [{s7}]");
    println!("-END DEMO 5-----\n");
}

fn change(some_string: &mut String) {
    println!("[change] ref has been passed in as some_string: [{some_string}]");
    some_string.push_str(", added");
    println!("[change] ref modified via push_str: [{some_string}]");
}

fn demo6() {
    println!("\n-START DEMO 6---");
    // Mutable references are singleton
    let mut s8 = String::from("s8");
    println!("s8 is in scope [{s8}]");

    let ref_s8 = &mut s8;
    println!("ref_s8 is in scope [{ref_s8}]");
    // Cannot do this! This would cause panic w/ two refs at once
    // let ref2_s8 = &mut s8;
    {
        // But we can inside of a different expression!
        let ref2_s8 = &mut s8;
        println!("  ref2_s8 is in scope [{ref2_s8}]");
        // This effectively destroys ref_s8
    }
    println!("all refs stale. Must use actual s8: [{s8}]");
    println!("-END DEMO 6-----\n");
}

fn demo7() {
    println!("\n-START DEMO 7---");
    let mut s9 = String::from("s9");
    // Lets make immutable refs
    let ref1 = &s9;
    let ref2 = &s9;
    println!("We have values for ref1 and ref2 respectively: [{ref1}] [{ref2}]");
    // Now that we do NOT need the above references...
    // Only now can we make a mutable ref!
    let ref3_mut = &mut s9;
    println!("We have value for ref3_mut: [{ref3_mut}]");
    change(ref3_mut);
    println!("We have value for ref3_mut: [{ref3_mut}]");
    println!("-END DEMO 7-----\n");
}

fn demo8() {
    println!("\n-START DEMO 8---");
    // This would break! Ref to dead value
    // let ref_to_nothing = dangle();
    let s10: String = no_dangle();
    println!("s10 is in memory: [{s10}]");
    println!("-END DEMO 8-----\n");
}

// fn dangle() -> &String {
//     let str = String::from("s10");
//     &str
// }
// Returning  a ref to a string that will fall out of memory!
// Here, s10 goes out of scope, and is dropped. Its memory goes away.
// Danger!
// Return actual String instead
// See fn no_dangle()

fn no_dangle() -> String {
    let s11 = String::from("s10");
    println!("[no_dangle] s10 has been created in memory: [{s11}]");
    s11
}

fn demo9() {
    println!("\n-START DEMO 9---");
    let mut s12 = String::from("s12 is the best");
    println!("s12 is in scope: [{s12}]");
    let word = first_words(&s12);
    s12.clear(); // This empties the String, making it ""
    println!("s12 has been emptied: [{s12}]...");
    println!("word has the value 3 here: [{word}]...");
    println!("we destroyued our context! s12: [{s12}] word: [{word}]");
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("-END DEMO 9-----\n");
}

fn first_words(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}

fn demo10() {
    println!("\n-START DEMO 10---");
    let s13 = String::from("s13 is the best");
    let ref_s13 = first_word_slice(&s13);
    println!("ref_s13: [{ref_s13}] and original string s13: [{s13}]");
    // The below code would BREAK the ref! Compiler would panic.
    // s13.clear();
    println!("-END DEMO 10-----\n");
}

fn first_word_slice(s: &str) -> &str {
    println!("[first_words_slice] received string ref [{s}]");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn demo11() {
    println!("\n-START DEMO 11---");
    let my_string = String::from("hello world");
    println!("my_string is a &String: [{my_string}]");
    println!("Let's log [word] every method call:\n");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    println!("word: [{word}]");
    let word = first_word_slice(&my_string[..]);
    println!("word: [{word}]");

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice(&my_string);
    println!("word: [{word}]");

    let my_string_literal = "hello world";
    println!("my_string_literal is a &str: [{my_string_literal}]");
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice(&my_string_literal[0..6]);
    println!("word: [{word}]");
    let word = first_word_slice(&my_string_literal[..]);
    println!("word: [{word}]");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);
    println!("word: [{word}]");
    println!("-END DEMO 11-----\n");
}

fn demo12() {
    println!("\n-START DEMO 12---");
    let a = [10,20,30,40,50];
    println!("a is an array with 5 items. Iterate:");
    for (i, &item) in a.iter().enumerate() {
        println!("Iteration item is: [{item}] at index: [{i}]");
    }
    let slice = &a[1..3];
    assert_eq!(slice, &[20, 30]);
    println!("slice is an array ref with 2 items. Iterate:");
    for (i, &item) in slice.iter().enumerate() {
        println!("Iteration item is: [{item}] at index: [{i}]");
    }
    println!("-END DEMO 12-----\n");
}