mod r#box;
mod rc;
mod deref;
mod ref_cell;

use std::ops::Deref;
pub(crate) use r#box::understand_box;
use crate::deref::MyBox;
use crate::rc::understand_rc;
use crate::ref_cell::Person;

fn main()
{
    understand_box();

    println!("\n======================\n");

    understand_rc();

    println!("\n======================\n");

    //deref trait uses to create own smart pointer
    //It will ve stored on heap
    let my_box: MyBox<Vec<i32>> = MyBox::new(vec![1, 3, 4]);

    //Here's a problem refer is not mutable
    //to mutable that use deref mut trait
    //we will see that in the next part

    println!("refer - {:?}, myBox = {:?}", *my_box.deref(), my_box);


    //Refcell
    let person = Person::new(0, "Tah-mid".to_string(), 9, "tm.ahad.07@gmail.com".to_string());

    let mut person_id = person.id.borrow_mut();
    *person_id = 2;

    println!("Before dropping - {:?}", person);

    drop(person_id);

    println!("After dropping: {:?}", person);
}
