use std::cell::RefCell;

#[derive(Debug)]
pub(crate) struct Person
{
    pub id: RefCell<u32>,
    pub name: String,
    pub age: u8,
    pub email: String
}

impl Person
{

    pub fn new(id_gen: u32, name: String, age: u8, email: String) -> Self
    {
        Self
        {
            id: RefCell::new(id_gen),
            name,
            age,
            email
        }
    }
}
