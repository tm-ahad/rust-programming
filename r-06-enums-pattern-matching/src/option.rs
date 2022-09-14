/*
Option in rust like optional it can be a value or None
Let's see
*/

pub fn understand_option() {
    let a: Option<String> = Some(String::from("Tah-mid"));

    match a {

        Some(val) => println!("a = {:?}", val),
        None => println!("a is none")
    }

    //see an another example

    let str: String = String::from("Tah-mid");

    for i in 0..8 {
        println!("matching pattern: {}", match str.chars().nth(i) {
            Some(v) => v,
            None => 'N'
        })
    }
}
