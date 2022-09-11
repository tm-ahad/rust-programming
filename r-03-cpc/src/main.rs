//variable and mutablity

// mod variables_mutabillity;
// mod data_types;
//
// pub(crate) use variables_mutabillity::{infinite_loop};

// fn main() {
//     infinite_loop(); //infinite loop ♾!
// }

//data_type
// mod data_types;
// mod other_data_types;
//
// pub(crate) use data_types::{dt};
//
// fn main() {
//
//     dt(); //printing 🖨️...
// }

//ot
// mod function;
// pub(crate) mod ifelse;
//
// pub(crate) use function::check_even;
//
// fn main() {
//     let u: u32 = 500;
//
//     println!("{}", check_even(u));
// }

//if..else statement

// mod ifelse;
//
// pub(crate) use ifelse::guess;
//
// fn main() {
//
//     for i in 0..3 {
//         println!("{}", guess(i));
//     }
// }

//while loop

// mod while_loop;
// mod recursion;
//
// pub(crate) use while_loop::while_loop;
// pub(crate) use recursion::fact;
//
// fn main() {
//     let vec: Vec<&str> = vec!["Banana", "Orange", "Apple", "Mango"];
//
//     while_loop(vec, 3);
//
// }

//recursion
mod recursion;
pub(crate) use recursion::fact;

fn main() {
    let num: i32 = 12;

    print!("{}", fact(num));
}