# RUST

## Description

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

> If you give someone a `.rb, .py, or .js` file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively)
> ...  
> ...but in those languages, you only need one command to compile and run your program.

**Everything is a trade-off in language design.**

## Examples

- 01_hello_world - Most basic program possible.
- 02_hello_cargo - Package management.
- 03_guessing_game - Variables, iteration, logic

## Package Manager

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

## Libraries

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program.  

This set is called the **prelude**, and you can see everything in it in the standard library documentation.  

> [Prelude Documentation](https://doc.rust-lang.org/std/prelude/index.html)