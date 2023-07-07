- [Rust-lang](#rust-lang)
  - [Description](#description)
  - [Package Management](#package-management)
    - [Commands](#commands)
    - [Libraries](#libraries)
    - [Crates](#crates)
  - [Concepts](#concepts)
    - [Evaluation](#evaluation)
    - [Debug](#debug)
    - [Ownership](#ownership)
    - [Structs](#structs)
    - [Enums](#enums)
    - [Modules](#modules)
    - [Collections](#collections)
      - [Vectors](#vectors)
      - [Strings](#strings)
      - [Hash Maps](#hash-maps)
    - [Error Handling](#error-handling)
    - [Abstract](#abstract)
      - [Generics](#generics)
      - [Traits](#traits)
      - [Validation](#validation)

# Rust-lang

## Description

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

> If you give someone a `.rb, .py, or .js` file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively)...  
> ...but in those languages, you only need one command to compile and run your program.

**Everything is a trade-off in language design.**

This repository is a sandbox to experiment while running through **The Rust Book**.

> [The Rust Book](https://doc.rust-lang.org/book)  

You can also explore further resources here:  

> [Learn Rust](https://www.rust-lang.org/learn)  

## Package Management

**Cargo** is Rust's build system and package manager. It handles tasks for you such as:
- building your code
- downloading the libraries your code depends on
- building those libraries
  - (We call the libraries that your code needs dependencies)

### Commands

- `cargo new <project name>` - Scaffolds new Rust project
- `cargo build` - Builds project
- `cargo run` - Runs build (and builds if needed)
- `cargo check` - Sanity check compilation (no executable generated)
- `cargo doc --open`  Generates documentation for all dependencies (and opens in browser)  

### Libraries

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program.  

This set is called the **prelude**, and you can see everything in it in the standard library documentation.  

[Prelude Documentation](https://doc.rust-lang.org/std/prelude/index.html)  

### Crates  

A crate is a collection of Rust source code files.  
This project contains many binary crates, which are executable.

- **Binary Crate** - Executable.
  - `src/main.rs` is root module binary crate
  - `bin/**/*.rs` are additional, executable binary crates
- **Libary Crate** - Contains code to be used in other programs. Not standalone.
  - `src/lib.rs` is a library crate for your binary
  - > You get 0 or 1 library crates, no more, no less!

The `Cargo.toml` file handles *library crate dependencies* and *binary crate definitions*.

External dependencies (library crates) are fetched from:  

https://crates.io/

## Concepts

[Common Rust Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

- [3_concepts](3_concepts/src/main.rs) - Variables, data types, functions, and control flow
- [4_ownership](4_ownership/src/main.rs) - Memory management and data integrity
- [5_structs](5_structs/src/main.rs) - Data structure 'struct' (object)
- [6_enums](6_enums/src/main.rs) - Enums and custom type categories
- [7_modules](7_modules/src/main.rs) - Creating modules and going over libraries
- [8_collections](8_collections/src/main.rs) - Iterating, text, and hash maps
- [9_error-handling](9_error-handling/src/main.rs) - Error handling

**To run a binary, enter the directory for `cargo` commands:**  
```rust
cd 4_concepts
cargo run
```

### Evaluation
```rust
// Statements are instructions that perform some action and do not return a value.
// They use semicolons as terminations.
let foo = "bar";
println!("baz");

// Expressions evaluate to a resultant value. Let's look at some examples.
// They do not use a semicolon, as they propagate the value.
3 == 3 -> true
42 + 1 -> 43
```
We explore many programming concepts in the concepts section.

### Debug

To effectively log structs, they must have a `Debug` attribute.
```rust
#[derive(Debug)]
struct StructName {
```
Individual properties will not stringify, as that takes ownership.  
`dbg!()` can only accept primitives or complex structures explicitly marked. It prints to the `stderr` stream.

`println!()` is another helpful debugging tool, but it prints to a different stream of `stdout`. It needs specific formatting to print structs:

```rust
// usage:
let some_struct = Rectangle {
    height: 10,
    width: 20
};
println!("label: {:?}", some_struct); // print
println!("label: {:#?}", some_struct); // pretty print
dbg!(some_struct); // stderr pretty print
```

### Ownership  

> [See Chapter 4](4_ownership/src/main.rs)  

**Ownership is a set of rules that govern how a Rust program manages memory.**

All programs have to manage the way they use a computer's memory while running.

- Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs.  
- In other languages, the programmer must explicitly allocate and free the memory.

**Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.**

If any of the rules are violated, the program won't compile.

> None of the features of ownership will slow down your program while it's running.

### Structs

> [See Chapter 5](5_ownership/src/main.rs)  

Structs operate much like objects on OOP.

**Structs**
```rust
struct User {
    active: bool,
    username: String,
    sign_in_count: u64
}

let user = User {
  active: true,
  username: String::from("username"),
  sign_in_count: 1
}
```

**Methods**

Methods are added to structs using the `impl` keyword.
We can make multiple `impl` decalarations and they all
will bind to the parent `struct`.  

```rust
struct SomeStruct {
    some_prop: u32
}

impl SomeStruct {
    // Does nothing special
    fn hello() {
        println!("hello, world!");
    }
    // Calcs using self reference
    fn can_hold(&self, some_other_struct: &SomeStruct) -> bool {
        self.some_prop > some_other_struct.some_prop
    }
    // Generates instance without instance
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}
```

### Enums

- [See Chapter 6](6_enums/src/main.rs) - Enums and custom type categories

We use enums to encode meaning along with data. Pattern matching is a critical part 
of programming in Rust.

> **Structs** group related fields and data like an `object`  
> **Enums** give you a way to define a `Set` of options  

```rust
enum Coin { Heads, Tails }
fn which_side(coin: Coin) {
    match coin {
        Coin::Tails => println!("tails"),
        Coin::Heads => println!("heads"),
        _ => panic!("what happened"),
    }
}
which_side(Coin::Heads); // "heads"
```

`if let` is syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.  

It may also contain an else as a catch all.

```rust
// Counting quarters NOT from a specific state
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

// Identical to
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

### Modules  

- [See Chapter 7](7_modules/src/main.rs) - Modules, packages, crates, and paths

A package can contain multiple binary crates and optionally one library crate.
As a package grows, you can extract parts into separate crates that become external dependencies
This chapter covers all these techniques.

> For **very large projects** comprising a set of interrelated packages that evolve together,
> Cargo provides workspaces, which we'll cover in the “Cargo Workspaces”
> section in Chapter 14.

**Terminology:**  

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and use: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

A package can contain as many binary crates as you like, but at most only one library crate!  
A package must contain at least one crate, whether that's a library or binary crate.

binary: `main.rs`
```rust
use my_library::eat_at_restaurant;
fn main() {
    eat_at_restaurant();
}
```

library: `lib.rs`
```rust
...
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    let meal = self::back_of_house::Breakfast::summer("Rye");
    let order = self::back_of_house::Appetizer::Soup;
    println!("{:?}{:?}", meal, order);
}
```  

The scope of modules can get quite large. For further reading, reference the workflows
or read the source material:

[Rust Lang Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)  

### Collections  

- [See Chapter 8](8_collections/src/main.rs) - Iterating, text, and hash maps

Most other data types represent one specific value, but collections can contain multiple values.

The data is stored on the heap, which means the amount of data does not need to be
known at compile time and can grow or shrink as the program runs.

Each kind of collection has different capabilities and costs (*tradeoffs*!), and choosing an appropriate one for your current situation is a skill you'll develop over time. 

- `vector`: allows you to store a variable number of values next to each other.
- `string`: is a collection of characters. We've mentioned the String type previously, but in this chapter we'll talk about it in depth.
- `hash map`: allows you to associate a value with a particular key. It's a particular implementation of the more general data structure called a map.

#### Vectors

The first collection type we'll look at is Vec<T>, also known as a vector.

Vectors allow you to store more than one value in a single data structure that puts all
the values next to each other in memory. Vectors can only store values of the same type.

They are useful when you have a list of items, such as the lines of text in a file
or the prices of items in a shopping cart.

> We can use `enums` to create a custom type that can hold many types, though!

#### Strings

We discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

We'll talk about the operations on `String` that every collection type has, such as creating, updating, and reading.

We'll also discuss the ways in which String is different from the other collections, namely
how indexing into a String is complicated by the differences between how people and computers
interpret String data.

```rust
// Ways to generate a string
let s0 = String::new();
let s1 = "initial contents";
let s2 = data.to_string();
let s3 = "initial contents".to_string();
let s4 = String::from("initial contents");
```

#### Hash Maps

Hash maps are useful when you want to look up data not by using an index, as you
can with vectors, but by using a key that can be of any type.

For example, in a game, you could keep track of each team’s score in a hash map
in which each key is a team’s name and the values are each team’s score. Given
a team name, you can retrieve its score.

```rust
fn hashmaps() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut scores = HashMap::new();

    scores.insert(&blue, 10);
    scores.insert(&yellow, 10);
    scores.insert(&blue, 10);
    scores.insert(&yellow, 50);
    scores.entry(&blue).or_insert(100);
    scores.entry(&yellow).or_insert(10);

    let home_score = scores.get(&blue);
    let visitor_score = scores.get(&yellow);
    println!("Home: {:?}\nVisitor: {:?}\n", home_score, visitor_score);
}
```

### Error Handling

- [See Chapter 9](9_error-handling/src/main.rs) - Error handling

Result type enum is:
- `Ok`
- `Err`

Quite simply, you are guaranteed to get one of the above responses in any `Result`, and each have differing methods.

Read documentation for `expect()` signature:

[expect() Docs](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)  

> Warnings will be thrown if `.expect()` is not chained onto `Result`

By default, we should use `Result` and error propagation, which allows the caller to best
handle the error.

> `panic()` is only used when it is an **irrecoverable** crash!  
> It will also cause the program to immediately terminate!
> Use `panic()` for examples or absolute crashes only!

For deeper error handling, we can use patterns like below:

You can use closures as well for a potentially terser version.

```rust
// Result
fn read_file() -> File {
    let path = String::from("./not-exist.txt");
    let f = File::open(&path);
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&path) {
                Ok(file_contents) => file_contents,
                Err(error) => panic!("Problem creating the file: {:?}", error)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    }
}
// Closures
fn read_file_closures() -> File {
    let path = String::from("./not-exist.txt");
    File::open(&path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}
```

### Abstract

This is a larger section comprised of:
- Generics
- Traits
- Validation

The general principles are difficult to bucket, so we have chosen to aggregate them into
an "abstract" category.

Every programming language has tools for effectively handling the duplication of concepts.  

We use **generics** to allow abstract types to be inferred so that the same code can produce
different functionality. Generics enable us to *generalize* parameters and properties.

We use **traits** to define behavior in a generic way. You can combine traits with generic types
to constrain a generic type to accept only those types that have a particular behavior, as
opposed to just any type.

We use **lifetimes** to allow us to give the compiler enough information about borrowed values so
that it can ensure references will be valid in more situations than it could without our help.

#### Generics

- [See Chapter 10-1](10_1_generics/src/main.rs) - Generics

We use generics to create definitions for items like function signatures or structs, which we can
then use with many different concrete data types.

> The `Option` and the `Result` enums use a **generic** `T` to infer a result type.  
> The `Result` enum also has a secondary **generic** `E` to infer an error type.

```rust
struct Point2<T> {
    x: T,
}
impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}
let point_int = Point { x: 42 }; // T type integer
let point_float = Point { x: 42.12 }; // T type float
let point_char = Point { x: 'c' }; // T type char
let point_str = Point { x: "String" }; // T type String
```

#### Traits  

- [See Chapter 10-2](10_2_traits/src/main.rs) - Traits

(tbd)

#### Validation

- [See Chapter 10-3](10_3_validation/src/main.rs) - Validation

(tbd)