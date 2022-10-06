mod r#box;
mod rc;
mod deref;
mod ref_cell;
mod r#drop;

use crate::r#drop::CustomSmartPointer;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
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

    //RefCell + Rc

    let string = String::from("Hello World!");
    let my_ref = Rc::new(RefCell::new(string));

    change_str(Rc::clone(&my_ref));

    println!("str - {:?}", Rc::clone(&my_ref));
    println!("ref_count - {}", Rc::strong_count(&my_ref));

    //drop_trait

    let a = CustomSmartPointer
    {
        data: "Some String".to_string()
    };

    drop(a);


}

pub fn change_str( refer: Rc<RefCell<String>> )
{
    let mut r = refer.borrow_mut();

    r.push('!');
}
