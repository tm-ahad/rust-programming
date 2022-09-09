use std::io::stdin;
use colored::Colorize;

pub fn welcome() {
    println!("{}{}", "Welcome to our ".yellow(), "guessing game".bold().italic().blue())
}

pub fn get_input_and_print() {
    let mut input: String = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You guessed: {}", input.purple().italic());

            get_input_and_print();
        },
        Err(e) => panic!("{}", e)
    }
}

pub fn main() {
    welcome();
    get_input_and_print();
}