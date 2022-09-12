// //shallow cloning
// mod shallow_clone;
// mod reference;
//
// pub(crate) use shallow_clone::under_stand_shallow_cloning;
//
// fn main() {
//     under_stand_shallow_cloning();
// }

//ref

mod reference;
pub(crate) use reference::under_stand_reference;

fn main() {
    under_stand_reference();
}