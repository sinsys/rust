- [RUST](#rust)
  - [Description](#description)
  - [Examples](#examples)
  - [Package Management](#package-management)
    - [Commands](#commands)
    - [Libraries](#libraries)
    - [Crates](#crates)
  - [Error Handling](#error-handling)

# RUST

## Description

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

> If you give someone a `.rb, .py, or .js` file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively)...  
> ...but in those languages, you only need one command to compile and run your program.

**Everything is a trade-off in language design.**

## Examples

- [01_hello_world](1_hello_world/main.rs) - Most basic program possible
- [02_hello_cargo](2_hello_cargo/src/main.rs) - Package management
- [03_guessing_game](3_guessing_game/src/main.rs) - Variables, iteration, logic

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

## Error Handling

Result type enum is:
- `Ok`
- `Err`

Quite simply, you are guaranteed to get one of the above responses in any `Result`, and each have differing methods.

Read documentation for `expect()` signature:

[expect() Docs](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)  

> Warnings will be thrown if `.expect()` is not chained onto `Result`

