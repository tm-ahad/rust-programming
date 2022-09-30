mod r#box;
mod rc;

pub(crate) use r#box::understand_box;
use crate::rc::understand_rc;

fn main()
{
    understand_box();

    println!("\n======================\n");

    understand_rc();
}