mod r#box;
mod rc;
mod deref;

use std::ops::Deref;
pub(crate) use r#box::understand_box;
use crate::deref::MyBox;
use crate::rc::understand_rc;

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
}