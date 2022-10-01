use std::ops::Deref;

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T>
{
    pub fn new(v: T) -> MyBox<T>
    {
        MyBox::<T>(v)
    }
}

impl<T> Deref for MyBox<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}