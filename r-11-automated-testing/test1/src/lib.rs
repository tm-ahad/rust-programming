
pub struct Rectangle
{
    id: u32,
    pub height: u32,
    pub width: u32
}

impl Rectangle
{
    pub fn new(id: u32, h: u32, w: u32) -> Self
    {
        return Self
        {
            id,
            height: h,
            width: w
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rect1: Rectangle = Rectangle::new(0, 7, 8);
        let b: bool = rect1.id == 0;

        assert_eq!(rect1.height, 7, "{}", "Test success...");
        assert!(b, "{}", "Test success...");
    }
}
