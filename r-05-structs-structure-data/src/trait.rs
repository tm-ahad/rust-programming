

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
    Brown,
    Chocolate,
    Pink,
    Violet
}

#[derive(Debug)]
pub struct Rectangle
{
    height: i128,
    width: i128,
    color:  Color,
    name: String
}


pub trait Common
{
    fn create(height: i128, width: i128, color: Color, name: String) -> Rectangle;
    fn get_area(&self) -> i128;
}

impl Common for Rectangle {
    fn create(height: i128, width: i128, color: Color, name: String) -> Rectangle
    {

        Rectangle {
            height,
            width,
            color,
            name
        }
    }

    fn get_area(&self) -> i128
    {
        return self.height * self.width
    }
}