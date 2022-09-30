use crate::r#box::List::Cons;

#[derive(Debug)]
pub enum List
{
    Cons(i32, Box<List>),
    Nil
}

pub fn understand_box()
{
    //box store a value in heap
    let b = Box::new(8);
    let list1 = Cons(8, Box::new(List::Nil));

    println!("{:?}", b);
    println!("{:?}", list1);
}