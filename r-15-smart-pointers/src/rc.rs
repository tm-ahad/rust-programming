use std::rc::Rc;
use crate::rc::List::{Cons, Nil};

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn understand_rc()
{

    let mut a = Rc::new(Cons(1, Rc::new(Nil)));
    let _b = Cons(3, Rc::clone(&a));

    println!("a is copied after creating b {} times", Rc::strong_count(&a));

    let _c = Cons(4, Rc::clone(&a));

    println!("a is copied after creating c {} times", Rc::strong_count(&a));

    println!("\nbefore changing a = {:?}, b = {:?}, c = {:?}", a, _b, _c);

    a = Rc::new(Cons(2, Rc::new(Nil)));

    println!("\nafter changing a = {:?}, b = {:?}, c = {:?}", a, _b, _c);
}