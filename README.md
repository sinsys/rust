- [Rust-lang](#rust-lang)
  - [Description](#description)
  - [Directories](#directories)
  - [Package Management](#package-management)
    - [Commands](#commands)
    - [Libraries](#libraries)
    - [Crates](#crates)
  - [Concepts](#concepts)
    - [Error Handling](#error-handling)
    - [Evaluation](#evaluation)
    - [Debug](#debug)
    - [Ownership](#ownership)
    - [Structs](#structs)
      - [Methods](#methods)

# Rust-lang

## Description

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

> If you give someone a `.rb, .py, or .js` file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively)...  
> ...but in those languages, you only need one command to compile and run your program.

**Everything is a trade-off in language design.**

> [Learn Rust](https://www.rust-lang.org/learn)  

## Directories

- [1_getting-started](1_getting-started) - Hello, world!
- [2_guessing_game](2_guessing-game/src/main.rs) - Basic project w/ variables, iteration, logic
- [3_concepts](3_concepts/src/main.rs) - Deeper variables, data types, functions, and control flow
- [4_ownership](4_ownership/src/main.rs) - Memory management and data integrity
- [5_structs](5_structs/src/main.rs) - Data structure 'struct' (object)

**To run a binary, enter the directory for `cargo` commands:**  
```rust
cd 4_concepts
cargo run
```

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
- **Libary Crate** - Contains code to be used in other programs. Not standalone.

The `Cargo.toml` file handles *library crate dependencies* and *binary crate definitions*.

External dependencies (library crates) are fetched from:  

https://crates.io/

## Concepts

[Common Rust Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

- [concepts](3_concepts/src/main.rs) - Deeper variables, data types, functions, and control flow
- [ownership](4_ownership/src/main.rs) - Memory management and data integrity
- [structs](5_structs/src/main.rs) - Data structure 'struct' (object)

### Error Handling

Result type enum is:
- `Ok`
- `Err`

Quite simply, you are guaranteed to get one of the above responses in any `Result`, and each have differing methods.

Read documentation for `expect()` signature:

[expect() Docs](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)  

> Warnings will be thrown if `.expect()` is not chained onto `Result`

### Evaluation
```rust
// Statements are instructions that perform some action and do not return a value.
// They use semicolons as terminations.
let foo = "bar";
println!("baz");

// Expressions evaluate to a resultant value. Let’s look at some examples.
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
println!("label: {:#?}", some_struct);
dbg!(some_struct);
```

### Ownership  

**Ownership is a set of rules that govern how a Rust program manages memory.**

All programs have to manage the way they use a computer's memory while running.

- Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs.  
- In other languages, the programmer must explicitly allocate and free the memory.

**Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.**

If any of the rules are violated, the program won’t compile.

> None of the features of ownership will slow down your program while it’s running.

### Structs

Structs operate much like objects on OOP.

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

#### Methods

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
