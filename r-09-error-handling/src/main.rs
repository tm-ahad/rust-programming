#![feature(str_split_whitespace_as_str)]

#![allow(unused)]
extern crate core;

use std::io::Error;

fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {

        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };


        let mut username = String::new();

        println!("{}", username);

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    let content = read_username_from_file();

    match content {
        Ok(val) => println!("{:?}", val),
        Err(e) => panic!("Err: {}", e)
    }
}
