// These flags suppress warnings
#![allow(dead_code)]
#![allow(unused_variables)]
use std::env::current_dir;
use std::fs::{File, read_to_string};
use std::io::{self, ErrorKind, Read};

fn main() {
    // Demo for crashing deeply
    // a();
    println!("{:?}", current_dir());
    // Paths are from project root, not src!
    let input = String::from("./working.txt");
    let contents = read_to_string(&input);
    println!("{:?}", contents);
    let contents_2 = read_file();
    let contents_3 = read_file_closures();
    let contents_4 = unwrap(&input);
    let username = read_username_from_file();
    println!("{:?}", username);
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(i: i32) {
    if i == 22 {
        // Can use RUST_BACKTRACE env flag to view full stack trace
        panic!("NO 22!");
    }
}

fn read_file() -> File {
    // We will try to read a path and setup a secondary path if not exist
    let path = String::from("./not-exist.txt");
    let path_if_not_exist = String::from("./now-exist.txt");

    // Attempt to open the file contents
    let f = File::open(&path);
    // We need a match statement since f is Result type
    let f = match f {
        // File was read and all is well
        Ok(file) => file,
        // File was NOT read. Let's use io specific ErrorKind!
        // error.kind() tries to match what type of error it is!
        Err(error) => match error.kind() {
            // If it is an io NotFound... create file!
            // This ALSO provides a Result type, so we must handle
            ErrorKind::NotFound => match File::create(&path_if_not_exist) {
                // New file was created successfully! 
                Ok(file_contents) => file_contents,
                Err(error) => panic!("Problem creating the file: {:?}", error)
            },
            // The error type was NOT a NotFound type!
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
    f
}

fn read_file_closures() -> File {
    // We will try to read a path and setup a secondary path if not exist
    let path = String::from("./not-exist.txt");
    let path_if_not_exist = String::from("./now-exist.txt");

    // Attempt to open the file contents
    File::open(&path).unwrap_or_else(|error| {
        // File was NOT read. Let's use io specific ErrorKind!
        // error.kind() tries to match what type of error it is!
        if error.kind() == ErrorKind::NotFound {
            // Create file!
            File::create(&path_if_not_exist).unwrap_or_else(|error| {
                // all other errors
                panic!("Problem creating the file: {:?}", error)
            })
        } else {
            // The error type was NOT a NotFound type!
            panic!("Problem opening the file: {:?}", error);
        }
    })
}

fn unwrap(path: &str) -> File {
    File::open(path)
        // only accepts a &str
        .expect("Problem opening the file")
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("./username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}