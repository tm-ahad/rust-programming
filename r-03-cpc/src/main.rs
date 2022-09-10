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
mod function;
pub(crate) use function::check_even;

fn main() {
    let u: u32 = 500;

    println!("{}", check_even(u));
}